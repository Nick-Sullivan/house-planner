import { Grid, Tabs } from "@mantine/core";
import React from "react";
import { useSearchParams } from "react-router";
import type { PaginatedResponseHouseResponse } from "~/client";
import { idParam, pageParam, searchParam } from "~/utils/pagination";
import { HouseListPanel } from "../HouseListPanel/HouseListPanel";
import { RequirementsPanel } from "../RequirementsPanel/RequirementsPanel";

export function HouseListLayout({
  houseResponse,
  onChange,
  children,
}: {
  houseResponse: PaginatedResponseHouseResponse;
  onChange: (id: number) => void;
  children: React.ReactNode;
}) {
  const [searchParams, setSearchParams] = useSearchParams();
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
    onChange(id);
  };
  return (
    <Grid
      h="100%"
      styles={{
        inner: { height: "100%" },
      }}
    >
      <Grid.Col
        span={4}
        py="md"
        style={{
          height: "100%",
          display: "flex",
          flexDirection: "column",
          outline: "1px solid var(--mantine-color-gray-3)",
        }}
      >
        {/* TODO: 95% is bad but I couldn't get it working any other way */}
        <Tabs defaultValue="requirements" h="95%">
          <Tabs.List grow>
            <Tabs.Tab value="requirements">Requirements</Tabs.Tab>
            <Tabs.Tab value="houses">Houses</Tabs.Tab>
          </Tabs.List>
          <RequirementsPanel />
          <HouseListPanel
            houseResponse={houseResponse}
            onIdChange={onIdChange}
            onPageChange={onPageChange}
          />
        </Tabs>
      </Grid.Col>
      <Grid.Col
        span={8}
        style={{
          padding: "0px",
          height: "100%",
          overflowY: "auto",
          // borderLeft: `1px solid var(--mantine-color-gray-3)`,
          backgroundColor: "red",
        }}
      >
        {children}
      </Grid.Col>
    </Grid>
  );
}
