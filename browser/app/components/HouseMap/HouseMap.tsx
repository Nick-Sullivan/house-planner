import React, { Suspense, useEffect } from "react";
import { Await } from "react-router";
import type { HouseResponse, PaginatedResponseHouseResponse } from "~/client";
import LoadingSpinner from "../LoadingSpinner/LoadingSpinner";
import { useMap } from "../MapContext/MapContext";

const HouseMap = ({
  mapRef,
  housesResponse,
  selectedHouse,
}: {
  mapRef: React.RefObject<google.maps.Map | null>;
  housesResponse: Promise<PaginatedResponseHouseResponse>;
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
      <Suspense>
        <Await resolve={housesResponse}>
          {(housesResponse) =>
            housesResponse.items.map((house: any) => (
              <leafletComponents.Marker
                key={house.id}
                position={[house.lat!, house.lon!]}
                opacity={selectedHouse?.id === house.id ? 1 : 0.3}
              />
            ))
          }
        </Await>
      </Suspense>
    </>
  );
};

export default HouseMap;
