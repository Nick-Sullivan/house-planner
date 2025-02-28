from enum import Enum

import contextily as ctx
import h3
import matplotlib.pyplot as plt
import mplcursors
import pyperclip
import requests
from shapely import MultiPoint, Point
from shapely.geometry import Polygon
from shapely.geometry.polygon import orient

from openstreetmaps.persistent_cache import persistent_cache


def get_adelaide_boundary() -> Polygon:
    overpass_url = "https://overpass-api.de/api/interpreter"
    overpass_query = """
    [out:json];
    relation["name"="Adelaide"]["admin_level"="7"]["boundary"="administrative"]["place"="city"]["type"="boundary"]["wikidata"="Q5112"]["wikipedia"="en:Adelaide"];
    out body;
    >;
    out skel;
    """
    response = requests.post(overpass_url, data={"data": overpass_query})
    data = response.json()
    # geojson = osm2geojson.json2geojson(data)
    # print(json.dumps(data, indent=2))
    elements = [element for element in data["elements"] if element["type"] == "node"]
    polygon_coords = [(coord["lon"], coord["lat"]) for coord in elements]
    points = MultiPoint(polygon_coords)
    polygon = points.convex_hull
    polygon = orient(polygon, sign=1.0)
    if polygon.is_valid:
        return polygon
    else:
        raise ValueError("Unable to create a valid polygon from the coordinates")


def get_h3_indices(polygon: Polygon, resolution: int) -> list[str]:
    geojson_polygon = {
        "type": "Polygon",
        "coordinates": [list(polygon.exterior.coords)],
    }
    h3_indices = h3.geo_to_cells(geojson_polygon, resolution)
    banned_indices = [
        "87b91694effffff",
        "87b916948ffffff",
        "87b91694bffffff",
        "87b91694affffff",
        "87b916864ffffff",
        "87b916865ffffff",
        "87b916959ffffff",
        "87b916866ffffff",
        "87b916860ffffff",
        "87b916863ffffff",
        "87b916862ffffff",
        "87b916875ffffff",
        "87b916871ffffff",
        "87b916844ffffff",
        "87b916846ffffff",
        "87b916842ffffff",
        "87b916855ffffff",
        "87b916851ffffff",
        "87b91685effffff",
        "87b9168edffffff",
        "87b9168e9ffffff",
        "87b9168e8ffffff",
        "87b9168ebffffff",
        "87b9168cdffffff",
        "87b9168ccffffff",
        "87b9168c8ffffff",
        "87b9168ceffffff",
        "87b916949ffffff",
        "87b91694dffffff",
        "87b91694cffffff",
        "87b91696bffffff",
        "87b9144b6ffffff",
        "87b916969ffffff",
        "87b91459affffff",
        "87b914598ffffff",
        "87b91459cffffff",
        "87b914583ffffff",
        "87b914580ffffff",
        "87b914585ffffff",
        "87b9145a3ffffff",
        "87b9145a1ffffff",
        "87b9168caffffff",
        "87b916164ffffff",
        "87b9168c3ffffff",
        "87b9168ddffffff",
        "87b9168dcffffff",
    ]

    return [i for i in h3_indices if i not in banned_indices]


def calculate_travel_durations(h3_indices: list[str], travel_mode: str, disk_size: int):
    durations = {}
    num_durations = num_cells_in_disk(disk_size) * len(h3_indices) / 2
    print(f"Calculating approximately {num_durations} travel durations")
    for source in h3_indices:
        for destination in h3.grid_disk(source, disk_size):
            if destination not in h3_indices or (source, destination) in durations:
                continue
            duration = (
                calculate_travel_duration(source, destination, travel_mode)
                if source != destination
                else 0
            )
            durations[(source, destination)] = duration
            durations[(destination, source)] = duration
    return durations


def num_cells_in_disk(disk_size: int):
    return 1 + 3 * disk_size * (disk_size + 1)


@persistent_cache(csv_file="travel_durations_cache.csv")
def calculate_travel_duration(source: str, destination: str, travel_mode: str):
    source_lat, source_lon = h3.cell_to_latlng(source)
    dest_lat, dest_lon = h3.cell_to_latlng(destination)
    # this api only supports driving
    url = f"http://router.project-osrm.org/route/v1/{travel_mode}/{source_lon},{source_lat};{dest_lon},{dest_lat}?overview=false"
    response = requests.get(url)
    data = response.json()
    if data["code"] == "Ok":
        duration = data["routes"][0]["duration"]
        return duration


def plot_adelaide_with_h3(
    polygon: Polygon, h3_indices: list[str], durations: dict[tuple[str, str], int]
):
    fig, ax = plt.subplots(figsize=(10, 10))

    # Add a background map using contextily
    # We need to set up the plot with the right coordinates first
    x_min, y_min, x_max, y_max = polygon.bounds
    ax.set_xlim(x_min, x_max)
    ax.set_ylim(y_min, y_max)
    ctx.add_basemap(
        ax,
        crs="EPSG:4326",  # WGS84 - standard coordinate system for lat/lng
        source=ctx.providers.CartoDB.Positron,  # Light map style
        attribution=False,  # Can set to True to show attribution
        alpha=1.0,  # Make the basemap slightly transparent
        zorder=0,  # Make sure it's behind the other elements
    )

    # Plot the Adelaide boundary
    x, y = polygon.exterior.xy
    # ax.plot(x, y, color="blue", label="Adelaide Boundary", linewidth=2, zorder=3)

    h3_polygons = []
    h3_patches = []  # Store polygon patches for hover detection

    # Create a colormap for travel times
    import matplotlib.cm as cm
    import matplotlib.colors as mcolors

    cmap = cm.get_cmap("viridis_r")  # Reversed viridis (yellow to purple)

    for h3_index in h3_indices:
        h3_boundary = h3.cell_to_boundary(h3_index)
        h3_polygon = Polygon(h3_boundary)

        # Important: Get the longitude (x) and latitude (y) in the correct order
        # h3.cell_to_boundary returns lat/lng, but matplotlib expects x/y (lng/lat)
        xy_points = [(lng, lat) for lat, lng in h3_boundary]

        # Create the polygon patch with these correctly ordered coordinates
        patch = plt.Polygon(
            xy_points,
            alpha=0.1,  # Slightly visible for hover detection
            fill=True,
            edgecolor="red",
            linestyle="--",
            linewidth=0.5,
            zorder=2,
            label="H3 Cell" if h3_index == h3_indices[0] else "",
        )
        ax.add_patch(patch)

        h3_polygons.append((patch, h3_index))
        h3_patches.append(patch)

    # Add colorbar for travel times
    norm = mcolors.Normalize(vmin=0, vmax=60)  # 0-30 minutes
    sm = plt.cm.ScalarMappable(cmap=cmap, norm=norm)
    sm.set_array([])
    cbar = plt.colorbar(sm, ax=ax, label="Travel Time (minutes)")

    ax.legend()
    ax.set_title("Adelaide Boundary with H3 Cells")
    ax.set_xlabel("Longitude")
    ax.set_ylabel("Latitude")
    ax.set_aspect("equal")

    def on_click(event):
        if event.inaxes != ax:
            return

        point = (event.xdata, event.ydata)

        # Find which H3 cell was clicked
        for i, (patch, h3_index) in enumerate(h3_polygons):
            # Get the H3 boundary again
            h3_boundary = h3.cell_to_boundary(h3_index)
            # Create a polygon from the boundary (lng, lat coordinates)
            polygon = Polygon([(lng, lat) for lat, lng in h3_boundary])

            # Check if the point is inside the polygon
            if polygon.contains(Point(point)):
                # Copy the H3 index to clipboard
                pyperclip.copy(h3_index)

                # Provide feedback
                print(f"{h3_index}")

                # Display temporary message on the plot
                # cell_center_x = patch.get_path().vertices[:, 0].mean()
                # cell_center_y = patch.get_path().vertices[:, 1].mean()
                # text = ax.text(
                # cell_center_x,
                # cell_center_y,
                # "Copied!",
                # ha="center",
                # va="center",
                # fontsize=12,
                # fontweight="bold",
                # color="green",
                # bbox=dict(facecolor="white", alpha=0.9, boxstyle="round,pad=0.3"),
                # zorder=15,
                # )

                # Remove the text after a short delay
                # plt.draw()
                # timer = fig.canvas.new_timer(interval=800)  # 800ms delay
                # timer.add_callback(lambda: remove_text(text))
                # timer.start()
                break

    # def remove_text(text):
    #     text.remove()
    #     fig.canvas.draw_idle()

    # Add this to help debug the hover detection
    def debug_hover(event):
        # Print where the cursor is
        if event.inaxes == ax:
            print(f"Cursor position: x={event.xdata}, y={event.ydata}")

            # Check each polygon
            for i, (patch, h3_index) in enumerate(h3_polygons):
                if patch.contains_point((event.xdata, event.ydata)):
                    print(f"Hovering over polygon {i}: {h3_index}")
                    break

    def hover(event):
        if event.inaxes != ax:
            return

        # Remove any text annotations
        for artist in ax.texts[1:]:  # Skip the title
            artist.remove()

        # Reset all patches to transparent
        for patch in h3_patches:
            patch.set_alpha(0.1)
            patch.set_facecolor("none")

        # Check which h3 cell contains the hover point
        hover_patch = None
        hovered_h3_index = None
        point = (event.xdata, event.ydata)

        # Store original polygons along with patches
        for i, (patch, h3_index) in enumerate(h3_polygons):
            # Get the H3 boundary again
            h3_boundary = h3.cell_to_boundary(h3_index)
            # Create a polygon from the boundary (lng, lat coordinates)
            polygon = Polygon([(lng, lat) for lat, lng in h3_boundary])

            # Check if the point is inside the polygon
            if polygon.contains(Point(point)):
                hover_patch = patch
                hovered_h3_index = h3_index
                break

        if hover_patch is None:
            fig.canvas.draw_idle()
            return

        # Highlight the hovered cell
        hover_patch.set_alpha(0.0)
        hover_patch.set_facecolor("red")
        # cell_center_x = hover_patch.get_path().vertices[:, 0].mean()
        # cell_center_y = hover_patch.get_path().vertices[:, 1].mean()
        # ax.text(
        #     cell_center_x,
        #     cell_center_y,
        #     f"H3: {hovered_h3_index}",
        #     ha="center",
        #     va="center",
        #     fontsize=9,
        #     fontweight="bold",
        #     bbox=dict(facecolor="white", alpha=0.9, boxstyle="round,pad=0.3"),
        #     zorder=10,
        # )
        # Update all other cells with color based on travel duration
        for patch, h3_index in h3_polygons:
            if h3_index != hovered_h3_index:
                # Check if duration data exists for this pair
                duration = durations.get((h3_index, hovered_h3_index))
                if duration is not None:
                    # Convert seconds to minutes
                    minutes = duration / 60

                    # Set color based on duration
                    color = cmap(norm(minutes))
                    patch.set_facecolor(color)
                    patch.set_alpha(0.2)  # More visible when showing durations

        # Force redraw to make sure everything appears
        fig.canvas.draw_idle()

    def leave_figure(event):
        # Reset all patches to transparent and remove any text annotations
        for patch in h3_patches:
            patch.set_alpha(0.1)
            patch.set_facecolor("none")

        # Remove any text annotations
        for artist in ax.texts[1:]:  # Skip the title
            artist.remove()

        fig.canvas.draw_idle()

    # Connect to mouse motion event for hover
    cid_motion = fig.canvas.mpl_connect("motion_notify_event", hover)
    cid_leave = fig.canvas.mpl_connect("figure_leave_event", leave_figure)
    cid_click = fig.canvas.mpl_connect("button_press_event", on_click)

    # Uncomment this to debug hover detection
    # fig.canvas.mpl_connect('motion_notify_event', debug_hover)

    plt.show(block=True)


if __name__ == "__main__":
    print("Getting boundary")
    boundary = get_adelaide_boundary()
    print("Getting h3")
    h3_indices = get_h3_indices(boundary, resolution=7)
    print("Calculating travel")
    durations = calculate_travel_durations(h3_indices, "driving", disk_size=7)
    print("Plotting")
    plot_adelaide_with_h3(boundary, h3_indices, durations)
