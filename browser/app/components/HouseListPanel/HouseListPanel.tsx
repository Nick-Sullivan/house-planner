import { Center, Divider, Pagination, ScrollArea, Tabs } from "@mantine/core";
import type { PaginatedResponseHouseResponse } from "~/client";
import { HouseCard } from "../HouseCard/HouseCard";

export function HouseListPanel({
  houseResponse,
  selectedHouseId,
  onIdChange,
  onPageChange,
}: {
  houseResponse: PaginatedResponseHouseResponse;
  selectedHouseId: number | null;
  onIdChange: (id: number) => void;
  onPageChange: (id: number) => void;
}) {
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
      {/* <SearchBox searchParam={searchParam} onChange={onSearchChange} /> */}
      <ScrollArea scrollbars="y" style={{ flex: 1 }}>
        {houseResponse.items.map((house) => (
          <HouseCard
            key={house.id}
            house={house}
            active={selectedHouseId?.toString() === house.id.toString()}
            onClick={onIdChange}
          />
        ))}
      </ScrollArea>
      <Divider my="xs" variant="dotted" />
      <Center>
        <Pagination
          color="blue.4"
          size="xs"
          value={houseResponse.currentPage}
          total={houseResponse.totalPages}
          onChange={onPageChange}
        />
      </Center>
    </Tabs.Panel>
  );
}
