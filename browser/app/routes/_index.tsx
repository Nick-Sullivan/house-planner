import { Tabs } from "@mantine/core";
import { GoogleMap, Marker, Polygon } from "@react-google-maps/api";
import { AdvancedMarker } from "@vis.gl/react-google-maps";
import * as h3 from "h3-js";
import { Suspense, useEffect, useRef, useState } from "react";
import {
  Await,
  useLoaderData,
  useSearchParams,
  type LoaderFunctionArgs,
} from "react-router";
import {
  Configuration,
  HouseApi,
  MapApi,
  type HouseResponse,
  type MapResponse,
} from "~/client";
import { HouseListPanel } from "~/components/HouseListPanel/HouseListPanel";
import LoadingSpinner from "~/components/LoadingSpinner/LoadingSpinner";
import { RequirementsPanel } from "~/components/RequirementsPanel/RequirementsPanel";
import { TwoColumnLayout } from "~/components/TwoColumnLayout/TwoColumnLayout";
import { idParam, pageParam, searchParam } from "~/utils/pagination";
import type { Route } from "../+types/root";

const ADELAIDE_CENTRE = { lat: -34.92866, lng: 138.59863 };

export const clientLoader = async ({ request, params }: LoaderFunctionArgs) => {
  const url = new URL(request.url);
  const page = url.searchParams.get(pageParam);
  const apiUrl = import.meta.env.VITE_API_URL;
  const config = new Configuration({ basePath: apiUrl });
  const houseApi = new HouseApi(config);
  const mapApi = new MapApi(config);
  const housesResponse = houseApi.getHouses({
    page: page ? parseInt(page) : undefined,
    pageSize: 50,
  });
  const mapResponse = await mapApi.getMap({
    mapRequest: { cityCode: "Adelaide", requirementIds: [] },
  });
  console.log(mapResponse);
  return { housesResponse, mapResponse };
};

export function meta({}: Route.MetaArgs) {
  return [
    { title: "House Planner" },
    { name: "description", content: "Welcome to React Router!" },
  ];
}

const lookupHouse = (
  houses: HouseResponse[],
  id: number | null
): HouseResponse | null => {
  return houses.find((house) => house.id === id) || null;
};

// const generateH3Grid = () => {
//   const polygon = [
//     [-34.85490824066172, 138.51536049346163],
//     [-34.84255603861368, 138.6595633990527],
//     [-34.95834949048342, 138.6873056569005],
//     [-34.96855376947172, 138.5194630253865],
//     // [138.51536049346163, -34.85490824066172],
//   ];
//   const hexagons = h3.polygonToCells(polygon, 7);
//   const boundaries = hexagons.map((hex) =>
//     h3.cellToBoundary(hex).map(([lat, lng]) => ({ lat, lng }))
//   );
//   return boundaries;
// };

const generateH3Grid = (mapResponse: MapResponse) => {
  const boundaries = mapResponse.tiles.map((tile) =>
    h3.cellToBoundary(tile.h3Index).map(([lat, lng]) => ({ lat, lng }))
  );
  return boundaries;
};

export default function Home() {
  const { housesResponse, mapResponse } = useLoaderData<typeof clientLoader>();
  const [_, setSearchParams] = useSearchParams();
  const [selectedHouse, setSelectedHouse] = useState<HouseResponse | null>();
  const [activeTab, setActiveTab] = useState<String | null>("requirements");
  const mapRef = useRef<google.maps.Map | null>(null);
  const h3Grid = generateH3Grid(mapResponse);
  useEffect(() => {
    if (selectedHouse && mapRef.current) {
      mapRef.current.panTo({
        lat: selectedHouse.lat!,
        lng: selectedHouse.lon!,
      });
    }
  }, [selectedHouse]);

  const onSearchChange = (value: string) => {
    setSearchParams((prev) => {
      prev.set(searchParam, value);
      prev.set(pageParam, "1");
      return prev;
    });
  };

  const onPageChange = (newPage: number) => {
    setSearchParams((prev) => {
      prev.set(pageParam, `${newPage}`);
      return prev;
    });
  };

  const onIdChange = (id: number) => {
    setSearchParams((prev) => {
      prev.set(idParam, `${id}`);
      return prev;
    });
    housesResponse.then((response) => {
      const house = lookupHouse(response.items, id);
      setSelectedHouse(house);
    });
  };

  return (
    <TwoColumnLayout
      leftPanel={
        <Tabs defaultValue="requirements" h="95%" onChange={setActiveTab}>
          <Tabs.List grow>
            <Tabs.Tab value="requirements">Requirements</Tabs.Tab>
            <Tabs.Tab value="houses">Houses</Tabs.Tab>
          </Tabs.List>
          <RequirementsPanel />
          <Suspense fallback={<LoadingSpinner />}>
            <Await resolve={housesResponse}>
              {(housesResponse) => (
                <HouseListPanel
                  houseResponse={housesResponse}
                  selectedHouseId={selectedHouse?.id || null}
                  onIdChange={onIdChange}
                  onPageChange={onPageChange}
                />
              )}
            </Await>
          </Suspense>
        </Tabs>
      }
    >
      <GoogleMap
        key="map"
        mapContainerStyle={{ width: "100%", height: "100%" }}
        center={ADELAIDE_CENTRE}
        zoom={14}
        options={{
          gestureHandling: "greedy",
          disableDefaultUI: true,
        }}
        onLoad={(map) => {
          mapRef.current = map;
        }}
      >
        {h3Grid.map((hex, index) => {
          return (
            <Polygon
              key={index}
              paths={hex}
              options={{
                fillColor: "#FF0000",
                fillOpacity: 0.2,
                strokeColor: "#FF0000",
                strokeOpacity: 0.5,
                strokeWeight: 2,
              }}
            />
          );
        })}
        {activeTab === "houses" && (
          <Suspense>
            <Await resolve={housesResponse}>
              {(housesResponse) =>
                housesResponse.items.map((house) => (
                  // <AdvancedMarker
                  //   key={house.id}
                  //   position={{ lat: house.lat!, lng: house.lon! }}
                  //   // opacity={selectedHouse?.id === house.id ? 1 : 0.3}
                  // />

                  <Marker
                    key={house.id}
                    position={{ lat: house.lat!, lng: house.lon! }}
                    opacity={selectedHouse?.id === house.id ? 1 : 0.3}
                  />
                ))
              }
            </Await>
          </Suspense>
        )}
      </GoogleMap>
    </TwoColumnLayout>
  );
}
