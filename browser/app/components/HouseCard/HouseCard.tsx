import { Card } from "@mantine/core";
import type { HouseResponse } from "~/client";

export function HouseCard({
  house,
  active,
  onClick,
}: {
  house: HouseResponse;
  active: boolean;
  onClick: (address: string) => void;
}) {
  return (
    <Card
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
}
