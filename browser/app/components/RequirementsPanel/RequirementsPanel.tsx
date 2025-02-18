import { Button, Card, ScrollArea, Tabs, Text } from "@mantine/core";
import { useState } from "react";
import { v4 as uuidv4 } from "uuid";
import {
  RequirementCard,
  type Requirement,
} from "../RequirementCard/RequirementCard";

function newRequirement(): Requirement {
  return {
    id: uuidv4(),
    duration: 30,
    travelType: "drive",
    address: null,
    lat: null,
    lng: null,
  };
}
export function RequirementsPanel({}: {}) {
  const [requirements, setRequirements] = useState<Requirement[]>([]);
  const addCard = () => {
    setRequirements([...requirements, newRequirement()]);
  };
  const deleteCard = (id: string) => {
    setRequirements(requirements.filter((req) => req.id !== id));
  };
  const updateCard = (req: Requirement) => {
    setRequirements(requirements.map((r) => (r.id === req.id ? req : r)));
  };
  const onCalculate = () => {
    console.log("Calculating");
  };

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
      <div style={{ display: "flex", justifyContent: "center", marginTop: 10 }}>
        <Button onClick={onCalculate} size="xs" variant="outline" color="gray">
          Calculate
        </Button>
      </div>
      <ScrollArea style={{ flex: 1, marginTop: 10 }}>
        {requirements.map((requirement, index) => (
          <RequirementCard
            key={index}
            defaultRequirement={requirement}
            onDelete={deleteCard}
            onChange={updateCard}
          />
        ))}
        <div
          style={{ display: "flex", justifyContent: "center", marginTop: 10 }}
        >
          <Button onClick={addCard} size="xs" variant="outline" color="gray">
            Add
          </Button>
        </div>
      </ScrollArea>
    </Tabs.Panel>
  );
}
