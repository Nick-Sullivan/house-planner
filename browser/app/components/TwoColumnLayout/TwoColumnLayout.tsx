import { Grid } from "@mantine/core";
import React from "react";

export function TwoColumnLayout({
  leftPanel,
  children,
}: {
  leftPanel: React.ReactNode;
  children: React.ReactNode;
}) {
  return (
    <Grid
      h="100%"
      styles={{
        inner: {
          height: "100%",
          width: "100%",
          margin: "0",
          boxSizing: "border-box",
        },
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
        {leftPanel}
      </Grid.Col>
      <Grid.Col
        span={8}
        style={{
          padding: "0px",
          height: "100%",
          overflowY: "auto",
          // borderLeft: `1px solid var(--mantine-color-gray-3)`,
          // backgroundColor: "red",
        }}
      >
        {children}
      </Grid.Col>
    </Grid>
  );
}
