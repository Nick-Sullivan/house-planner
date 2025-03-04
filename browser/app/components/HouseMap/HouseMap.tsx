import React, { useEffect } from "react";
import type { HouseResponse } from "~/client";
import type { HousesState } from "~/hooks/useHouses";
import LoadingSpinner from "../LoadingSpinner/LoadingSpinner";
import { useMap } from "../MapContext/MapContext";

const HouseMap = ({
  mapRef,
  houses,
  selectedHouse,
}: {
  mapRef: React.RefObject<google.maps.Map | null>;
  houses: HousesState;
  selectedHouse: HouseResponse | null;
}) => {
  const leafletComponents = useMap();
  useEffect(() => {
    if (selectedHouse && mapRef.current) {
      mapRef.current.panTo({
        lat: selectedHouse.lat!,
        lng: selectedHouse.lon!,
      });
    }
  }, [selectedHouse]);

  if (!leafletComponents) {
    return <LoadingSpinner />;
  }
  return (
    <>
      {houses.items.map((house: HouseResponse) => (
        <leafletComponents.Marker
          key={house.address}
          position={[house.lat!, house.lon!]}
          opacity={selectedHouse?.address === house.address ? 1 : 0.3}
        />
      ))}
    </>
  );
};

export default HouseMap;
