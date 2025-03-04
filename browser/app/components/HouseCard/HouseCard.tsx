import { Card } from "@mantine/core";
import { forwardRef } from "react";
import type { HouseResponse } from "~/client";

interface HouseCardProps {
  house: HouseResponse;
  active: boolean;
  onClick: (address: string) => void;
}

export const HouseCard = forwardRef<HTMLDivElement, HouseCardProps>(
  ({ house, active, onClick }, ref) => {
    return (
      <Card
        ref={ref}
        shadow="xs"
        padding="md"
        m="xs"
        radius="md"
        withBorder={active}
        onClick={() => onClick(house.address)}
        style={{ cursor: "pointer", borderWidth: "3px" }}
      >
        {house.address}
      </Card>
    );
  },
);

// Add displayName for better debugging
HouseCard.displayName = "HouseCard";
