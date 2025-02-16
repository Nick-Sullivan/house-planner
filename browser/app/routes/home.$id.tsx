import { Box, TextInput } from "@mantine/core";
import { Map, Marker, useMarkerRef } from "@vis.gl/react-google-maps";
import { map } from "radash";
import React, { useEffect, useMemo, useRef, useState, type JSX } from "react";
import {
  useLoaderData,
  useLocation,
  useParams,
  type LoaderFunctionArgs,
} from "react-router";
import { Configuration, HouseApi } from "~/client";

// export async function loader({ params }: LoaderFunctionArgs) {
//   const houseId = parseInt(params.id!);
//   const config = new Configuration({ basePath: "http://localhost:3000" });
//   const api = new HouseApi(config);
//   const house = await api.getHouseById({
//     id: houseId,
//   });
//   return { house };
// }

export default function HomeDetails() {
  const inputRef = useRef("");
  const [inputValue, setInputValue] = useState(() => inputRef.current);

  useEffect(() => {
    inputRef.current = inputValue;
  }, [inputValue]);

  const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setInputValue(event.target.value);
  };

  return (
    <Box style={{ width: "100%", height: "100%" }}>
      <TextInput value={inputValue} onChange={handleInputChange} />
    </Box>
  );
  // return (
  //   <Map
  //     key="map"
  //     style={{ width: "100%", height: "100%" }}
  //     // center={position}
  //     // defaultCenter={{ lat: house.lat!, lng: house.lon! }}
  //     defaultZoom={14}
  //     gestureHandling={"greedy"}
  //     disableDefaultUI={true}
  //   >
  //     {/* <Marker ref={markerRef} position={{ lat: house.lat!, lng: house.lon! }} /> */}
  //   </Map>
  // );
}
