import { Button, ScrollArea, Tabs } from "@mantine/core";
import { v4 as uuidv4 } from "uuid";
import { TravelMode } from "~/client";
import type { Requirement } from "~/utils/requirementUtils";
import { RequirementCard } from "../RequirementCard/RequirementCard";

function newRequirement(): Requirement {
  return {
    id: uuidv4(),
    duration: 30,
    travelType: TravelMode.Driving,
    location: null,
  };
}

export function RequirementsPanel({
  requirements,
  onChange,
  onDelete,
}: {
  requirements: Requirement[];
  onChange: (req: Requirement) => void;
  onDelete: (id: string) => void;
}) {
  return (
    <Tabs.Panel
      value="requirements"
      h="100%"
      style={{
        display: "flex",
        flexDirection: "column",
        flex: 1,
      }}
    >
      <ScrollArea style={{ flex: 1, marginTop: 10 }}>
        {requirements.map((req) => (
          <RequirementCard
            key={req.id}
            requirement={req}
            onDelete={onDelete}
            onChange={onChange}
          />
        ))}
        <div
          style={{ display: "flex", justifyContent: "center", marginTop: 10 }}
        >
          <Button
            onClick={() => onChange(newRequirement())}
            size="xs"
            variant="outline"
            color="gray"
          >
            Add
          </Button>
        </div>
      </ScrollArea>
    </Tabs.Panel>
  );
}
