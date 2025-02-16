import { Button, Card, ScrollArea, Tabs, Text } from "@mantine/core";
import { useState } from "react";
import { useSearchParams } from "react-router";
import { RequirementCard } from "../RequirementCard/RequirementCard";

export function RequirementsPanel({}: {}) {
  const [cards, setCards] = useState<number[]>([]);

  const addCard = () => {
    setCards([...cards, cards.length + 1]);
  };
  const deleteCard = (index: number) => {
    setCards(cards.filter((_, i) => i !== index));
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
      <ScrollArea style={{ flex: 1, marginTop: 10 }}>
        {cards.map((card, index) => (
          <RequirementCard
            key={index}
            id={index}
            address={card.toString()}
            onDelete={() => deleteCard(index)}
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
