import { Tabs } from "@mantine/core";
import { useEffect, useRef, useState } from "react";
import { useSearchParams } from "react-router";
import { type MapTileResponse } from "~/client";
import BaseMap from "~/components/BaseMap/BaseMap";
import { HouseListPanel } from "~/components/HouseListPanel/HouseListPanel";
import HouseMap from "~/components/HouseMap/HouseMap";
import { MapTextBox } from "~/components/MapTextBox/MapTextBox";
import RequirementsMap from "~/components/RequirementsMap/RequirementsMap";
import { RequirementsPanel } from "~/components/RequirementsPanel/RequirementsPanel";
import { TwoColumnLayout } from "~/components/TwoColumnLayout/TwoColumnLayout";
import { useHouses } from "~/hooks/useHouses";
import { useRequirements } from "~/hooks/useRequirements";
import { idParam } from "~/utils/constants";
import type { Route } from "../+types/root";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "House Planner" },
    { name: "description", content: "Welcome to React Router!" },
  ];
}

export default function Home() {
  const [_, setSearchParams] = useSearchParams();
  const [activeTab, setActiveTab] = useState<string | null>("requirements");
  const { housesData, selectedHouse, loadMore, selectHouse, handleTileSelect } =
    useHouses();
  const [hoveredTile, setHoveredTile] = useState<MapTileResponse | null>(null);
  const [selectedTile, setSelectedTile] = useState<MapTileResponse | null>(
    null,
  );
  const { requirements, map, onRequirementChange, onRequirementDelete } =
    useRequirements();
  const mapRef = useRef<google.maps.Map | null>(null);

  const onAddressChange = (address: string) => {
    setSearchParams((prev) => {
      prev.set(idParam, `${address}`);
      return prev;
    });
    selectHouse(address);
  };

  useEffect(() => {
    handleTileSelect(selectedTile);
  }, [selectedTile]);

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
          <HouseListPanel
            houses={housesData}
            selectedHouseAddress={selectedHouse?.address || null}
            onAddressChange={onAddressChange}
            onLoadMore={() => loadMore(selectedTile)}
          />
        </Tabs>
      }
    >
      <BaseMap
        mapRef={mapRef}
        map={map}
        onTileHover={setHoveredTile}
        onTileClick={setSelectedTile}
      >
        {activeTab === "requirements" ? (
          <RequirementsMap
            mapRef={mapRef}
            requirements={requirements}
            selectedRequirement={null}
          />
        ) : (
          <HouseMap
            mapRef={mapRef}
            houses={housesData}
            selectedHouse={selectedHouse}
          />
        )}
      </BaseMap>
      <MapTextBox tile={hoveredTile} requirements={requirements} />
    </TwoColumnLayout>
  );
}
