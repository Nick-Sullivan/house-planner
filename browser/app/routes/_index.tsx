import { Text } from "@mantine/core";
import { Map, Marker, useMap, useMarkerRef } from "@vis.gl/react-google-maps";
import {
  useLoaderData,
  useLocation,
  useParams,
  useSearchParams,
  type LoaderFunctionArgs,
} from "react-router";

import { useEffect, useState } from "react";
import { Configuration, HouseApi, type HouseResponse } from "~/client";
import { HouseListLayout } from "~/components/HouseListLayout/HouseListLayout";
import { idParam, pageParam } from "~/utils/pagination";
import type { Route } from "../+types/root";

const ADELAIDE_CENTRE = { lat: -34.92866, lng: 138.59863 };

export const clientLoader = async ({ request, params }: LoaderFunctionArgs) => {
  const url = new URL(request.url);
  const page = url.searchParams.get(pageParam);
  const apiUrl = import.meta.env.VITE_API_URL;
  // const config = new Configuration({
  //   basePath: "https://x9b98yw9z7.execute-api.eu-west-2.amazonaws.com/v1",
  // });
  const config = new Configuration({ basePath: apiUrl });
  const api = new HouseApi(config);
  const housesResponse = await api.getHouses({
    page: page ? parseInt(page) : undefined,
    pageSize: 50,
  });
  return { housesResponse };
};

export function meta({}: Route.MetaArgs) {
  return [
    { title: "House Planner" },
    { name: "description", content: "Welcome to React Router!" },
  ];
}

export default function Home() {
  const { housesResponse } = useLoaderData<typeof clientLoader>();
  const lookupHouse = (id: number | null): HouseResponse | null => {
    return housesResponse.items.find((house) => house.id === id) || null;
  };
  const [searchParams, _] = useSearchParams();
  const idParamValue = searchParams.get(idParam);
  const [selectedHouse, setSelectedHouse] = useState<HouseResponse | null>(
    lookupHouse(idParamValue ? parseInt(idParamValue) : null)
  );
  const map = useMap();
  useEffect(() => {
    if (!map) return;
    if (selectedHouse) {
      map.panTo({ lat: selectedHouse.lat!, lng: selectedHouse.lon! });
    }
  }, [selectedHouse]);

  return (
    <HouseListLayout
      houseResponse={housesResponse}
      onChange={(id) => setSelectedHouse(lookupHouse(id))}
    >
      <Map
        key="map"
        style={{ width: "100%", height: "100%" }}
        defaultCenter={ADELAIDE_CENTRE}
        defaultZoom={14}
        gestureHandling={"greedy"}
        disableDefaultUI={true}
      >
        {housesResponse.items.map((house) => (
          <Marker
            key={house.id}
            position={{ lat: house.lat!, lng: house.lon! }}
            opacity={selectedHouse?.id === house.id ? 1 : 0.3}
          />
        ))}
      </Map>
    </HouseListLayout>
  );
}
