import { ScrollArea, Tabs } from "@mantine/core";
import { useCallback } from "react";
import type { HousesState } from "~/hooks/useHouses";
import { HouseCard } from "../HouseCard/HouseCard";
import LoadingSpinner from "../LoadingSpinner/LoadingSpinner";

export function HouseListPanel({
  houses,
  selectedHouseAddress,
  onAddressChange,
  onLoadMore,
}: {
  houses: HousesState;
  selectedHouseAddress: string | null;
  onAddressChange: (id: string) => void;
  onLoadMore?: () => void;
}) {
  const lastItemRef = useCallback(
    (node: HTMLDivElement | null) => {
      if (!node || !houses.hasMore || houses.isLoading || !onLoadMore) return;
      const observer = new IntersectionObserver(
        (entries) => {
          if (entries[0].isIntersecting && houses.hasMore) {
            onLoadMore();
            observer.disconnect();
          }
        },
        { threshold: 0.1 },
      );
      observer.observe(node);
      return () => observer.disconnect();
    },
    [houses.hasMore, houses.isLoading, onLoadMore],
  );

  return (
    <Tabs.Panel
      value="houses"
      h="100%"
      style={{
        display: "flex",
        flexDirection: "column",
        flex: 1,
      }}
    >
      <ScrollArea scrollbars="y" style={{ flex: 1 }}>
        {houses.items.map((house, index) => {
          const isLast = index === houses.items.length - 1;
          return (
            <HouseCard
              key={house.address}
              ref={isLast ? lastItemRef : undefined}
              house={house}
              active={
                selectedHouseAddress?.toString() === house.address.toString()
              }
              onClick={onAddressChange}
            />
          );
        })}
        {houses.isLoading && <LoadingSpinner />}
      </ScrollArea>
    </Tabs.Panel>
  );
}
