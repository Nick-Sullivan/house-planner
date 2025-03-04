import { useState } from "react";
import type {
  GetHousesRequest,
  HouseResponse,
  MapTileResponse,
} from "~/client";
import { houseApi } from "~/utils/apiClient";

export interface HousesState {
  items: HouseResponse[];
  lastEvaluatedKey: string | null;
  isLoading: boolean;
  hasMore: boolean;
}

export function useHouses({
  initialLimit = 10,
}: { initialLimit?: number } = {}) {
  const [selectedHouse, setSelectedHouse] = useState<HouseResponse | null>(
    null,
  );
  const [housesData, setHousesData] = useState<HousesState>({
    items: [],
    lastEvaluatedKey: null,
    isLoading: false,
    hasMore: true,
  });

  function loadMore(selectedTile: MapTileResponse | null) {
    if (housesData.isLoading || !housesData.hasMore || !selectedTile) {
      return;
    }
    console.log("Loading more houses");
    setHousesData({ ...housesData, isLoading: true });
    const request: GetHousesRequest = {
      h3Index: selectedTile.h3Index,
      limit: initialLimit,
      lastEvaluatedKey: housesData.lastEvaluatedKey
        ? housesData.lastEvaluatedKey
        : undefined,
    };
    houseApi
      .getHouses(request)
      .then((response) => {
        setHousesData({
          items: [...housesData.items, ...response.items],
          lastEvaluatedKey: response.lastEvaluatedKey || null,
          isLoading: false,
          hasMore:
            response.items.length >= initialLimit &&
            !!response.lastEvaluatedKey,
        });
      })
      .catch((error) => {
        console.error("Failed to load houses:", error);
        setHousesData({ ...housesData, isLoading: false });
      });
  }

  function handleTileSelect(tile: MapTileResponse | null) {
    if (!tile) {
      return;
    }
    console.log("handling tile select");

    setHousesData({
      items: [],
      lastEvaluatedKey: null,
      isLoading: true,
      hasMore: true,
    });
    houseApi
      .getHouses({
        h3Index: tile.h3Index,
        limit: initialLimit,
      })
      .then((response) => {
        setHousesData({
          items: response.items,
          lastEvaluatedKey: response.lastEvaluatedKey || null,
          isLoading: false,
          hasMore:
            response.items.length >= initialLimit &&
            !!response.lastEvaluatedKey,
        });
      })
      .catch((error) => {
        console.error("Failed to load houses:", error);
        setHousesData({
          items: [],
          lastEvaluatedKey: null,
          isLoading: false,
          hasMore: false,
        });
      });
  }

  function lookupHouse(address: string | null): HouseResponse | null {
    if (!address) return null;
    return housesData.items.find((house) => house.address === address) || null;
  }

  function selectHouse(address: string) {
    console.log("selecting house");
    const house = lookupHouse(address);
    setSelectedHouse(house);
  }

  return {
    housesData,
    selectedHouse,
    loadMore,
    selectHouse,
    lookupHouse,
    handleTileSelect,
  };
}
