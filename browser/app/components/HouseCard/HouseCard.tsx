import { Card, Group, Text, Title } from "@mantine/core";
import { IconBath, IconBed, IconCar } from "@tabler/icons-react";
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
        <Title order={5}>{house.address}</Title>
        <Group mt="md">
          <Group gap="xs">
            <IconBed size={16} />
            <Text>{house.numBedrooms}</Text>
          </Group>
          <Group gap="xs">
            <IconBath size={16} />
            <Text>{house.numBathrooms}</Text>
          </Group>
          <Group gap="xs">
            <IconCar size={16} />
            <Text>{house.numCarspaces}</Text>
          </Group>
          <Text>{house.propertyType}</Text>
        </Group>
      </Card>
    );
  },
);

HouseCard.displayName = "HouseCard";
