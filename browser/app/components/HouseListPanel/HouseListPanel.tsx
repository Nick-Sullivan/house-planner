import { Center, Divider, Pagination, ScrollArea, Tabs } from "@mantine/core";
import { useSearchParams } from "react-router";
import type { PaginatedResponseHouseResponse } from "~/client";
import { idParam } from "~/utils/pagination";
import { HouseCard } from "../HouseCard/HouseCard";

export function HouseListPanel({
  houseResponse,
  onIdChange,
  onPageChange,
}: {
  houseResponse: PaginatedResponseHouseResponse;
  onIdChange: (id: number) => void;
  onPageChange: (id: number) => void;
}) {
  const [searchParams, setSearchParams] = useSearchParams();
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
            active={searchParams.get(idParam) === house.id.toString()}
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
