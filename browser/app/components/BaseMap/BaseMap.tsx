import * as h3 from "h3-js";
import "leaflet/dist/leaflet.css";
import { useState } from "react";
import type { MapResponse, MapTileResponse } from "~/client";
import LoadingSpinner from "~/components/LoadingSpinner/LoadingSpinner";
import { useMap } from "~/components/MapContext/MapContext";
import { MapTextBox } from "../MapTextBox/MapTextBox";

const ADELAIDE_CENTRE = { lat: -34.92866, lng: 138.59863 };

// const getTileColor = (score: number) => {
//   const red = Math.round(Math.min(255, Math.max(0, 255 - score * 2.55)));
//   const green = Math.round(Math.min(255, Math.max(0, score * 2.55)));
//   return `rgb(${red},${green},0)`;
// };

const tileColors = [
  "#8154fc",
  "#007eff",
  "#009ae6",
  "#00a8db",
  "#00b6d8",
  "#00c5d1",
  "#00d6c2",
  "#00e9a5",
  "#58f484",
  "#acfa70",
];
const tileDivisor = 100 / (tileColors.length - 1);

const getTileColor = (score: number): string => {
  return "#acfa70";
  // const color = tileColors[Math.floor(score / tileDivisor)];
  // return color;
};

const getTileOpacity = (score: number): number => {
  const max = 0.4;
  const min = 0;
  const opacity = (max - min) * (score / 100) + min;
  return opacity;
};
export default function BaseMap({
  map,
  mapRef,
  children,
  onTileHover,
}: {
  map: MapResponse | null;
  mapRef: React.RefObject<google.maps.Map | null>;
  children: React.ReactNode;
  onTileHover: (tile: MapTileResponse | null) => void;
}) {
  const leafletComponents = useMap();
  const [hoveredTile, setHoveredTile] = useState<string | null>(null);

  if (!leafletComponents) {
    return <LoadingSpinner />;
  }

  const handleMouseOver = (index: number, tile: MapTileResponse) => {
    setHoveredTile(tile.h3Index);
    onTileHover(tile);
  };

  const handleMouseOut = (index: number) => {
    setHoveredTile(null);
    onTileHover(null);
  };

  return (
    <leafletComponents.MapContainer
      center={ADELAIDE_CENTRE}
      zoom={12}
      style={{ width: "100%", height: "100%" }}
      ref={mapRef}
    >
      <leafletComponents.TileLayer url="https://{s}.basemaps.cartocdn.com/light_all/{z}/{x}/{y}.png" />
      {map &&
        map.tiles.map((tile: MapTileResponse, index: number) => (
          <leafletComponents.Polygon
            key={index}
            positions={h3
              .cellToBoundary(tile.h3Index)
              .map(([lat, lng]: [number, number]) => [lat, lng])}
            pathOptions={{
              fillColor: getTileColor(tile.meanScore),
              fillOpacity: getTileOpacity(tile.meanScore),
              weight: tile.h3Index === hoveredTile ? 3 : 0,
            }}
            eventHandlers={{
              mouseover: () => handleMouseOver(index, tile),
              mouseout: () => handleMouseOut(index),
            }}
          />
        ))}
      {children}
    </leafletComponents.MapContainer>
  );
}
