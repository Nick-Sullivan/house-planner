import { Tabs } from "@mantine/core";
import { Suspense, useEffect, useMemo, useRef, useState } from "react";
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
  type MapTileResponse,
} from "~/client";
import BaseMap from "~/components/BaseMap/BaseMap";
import { HouseListPanel } from "~/components/HouseListPanel/HouseListPanel";
import HouseMap from "~/components/HouseMap/HouseMap";
import LoadingSpinner from "~/components/LoadingSpinner/LoadingSpinner";
import { MapTextBox } from "~/components/MapTextBox/MapTextBox";
import RequirementsMap from "~/components/RequirementsMap/RequirementsMap";
import { RequirementsPanel } from "~/components/RequirementsPanel/RequirementsPanel";
import { TwoColumnLayout } from "~/components/TwoColumnLayout/TwoColumnLayout";
import { houseApi, mapApi } from "~/utils/apiClient";
import { idParam, pageParam, searchParam } from "~/utils/constants";
import {
  isCompletedRequirement,
  requirementToRequest,
  type Requirement,
} from "~/utils/requirementUtils";
import type { Route } from "../+types/root";

export const clientLoader = async ({ request, params }: LoaderFunctionArgs) => {
  const url = new URL(request.url);
  const page = url.searchParams.get(pageParam);
  const housesResponse = houseApi.getHouses({
    page: page ? parseInt(page) : undefined,
    pageSize: 50,
  });
  const mapResponse = await mapApi.getMap({
    mapRequest: { cityCode: "Adelaide", requirementIds: [] },
  });
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
  id: number | null,
): HouseResponse | null => {
  return houses.find((house) => house.id === id) || null;
};

export default function Home() {
  const { housesResponse, mapResponse } = useLoaderData<typeof clientLoader>();
  const [_, setSearchParams] = useSearchParams();
  const [selectedHouse, setSelectedHouse] = useState<HouseResponse | null>(
    null,
  );
  const [activeTab, setActiveTab] = useState<string | null>("requirements");
  const [requirements, setRequirements] = useState<Requirement[]>([]);
  const [map, setMap] = useState(mapResponse);
  const [hoveredTile, setHoveredTile] = useState<MapTileResponse | null>(null);
  const mapRef = useRef<google.maps.Map | null>(null);

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

  const onRequirementChange = async (req: Requirement) => {
    console.log("Updating requirement");
    if (isCompletedRequirement(req)) {
      console.log("POSTing requirement");
      const reqRequest = requirementToRequest(req);
      await mapApi.postRequirement({ requirementRequest: reqRequest });
    }
    setRequirements((prevRequirements) => {
      const existingRequirement = prevRequirements.find((r) => r.id === req.id);
      if (existingRequirement) {
        return prevRequirements.map((r) => (r.id === req.id ? req : r));
      } else {
        return [...prevRequirements, req];
      }
    });
  };

  const onRequirementDelete = (id: string) => {
    console.log("Deleting requirement");
    console.log("DELETING requirement");
    const existingRequirement = requirements.find((r) => r.id === id);
    if (!existingRequirement) {
      return;
    }
    setRequirements((prevRequirements) =>
      prevRequirements.filter((r) => r.id !== id),
    );
    // TODO delete from backend
  };

  const completedRequirements = useMemo(
    () => requirements.filter(isCompletedRequirement),
    [requirements],
  );

  useEffect(() => {
    console.log("Getting a new map");
    mapApi
      .getMap({
        mapRequest: {
          cityCode: "Adelaide",
          requirementIds: completedRequirements.map((r) => r.id),
        },
      })
      .then((mapResponse) => {
        console.log("Got a new map");
        console.log(mapResponse);
        setMap(mapResponse);
      });
  }, [completedRequirements]);

  return (
    <TwoColumnLayout
      leftPanel={
        <Tabs defaultValue="requirements" h="95%" onChange={setActiveTab}>
          <Tabs.List grow>
            <Tabs.Tab value="requirements">Requirements</Tabs.Tab>
            <Tabs.Tab value="houses">Houses</Tabs.Tab>
          </Tabs.List>
          <RequirementsPanel
            requirements={requirements}
            onChange={onRequirementChange}
            onDelete={onRequirementDelete}
          />
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
      <BaseMap mapRef={mapRef} map={map} onTileHover={setHoveredTile}>
        {activeTab === "requirements" ? (
          <RequirementsMap
            mapRef={mapRef}
            requirements={requirements}
            selectedRequirement={null}
          />
        ) : (
          <HouseMap
            mapRef={mapRef}
            housesResponse={housesResponse}
            selectedHouse={selectedHouse}
          />
        )}
      </BaseMap>
      <MapTextBox tile={hoveredTile} requirements={requirements} />
    </TwoColumnLayout>
  );
}
