import {
  ActionIcon,
  Card,
  Group,
  Select,
  Text,
  TextInput,
  type ComboboxData,
} from "@mantine/core";
import { Autocomplete } from "@react-google-maps/api";
import { IconTrash } from "@tabler/icons-react";
import { useState } from "react";

export type Requirement = {
  id: string;
  duration: number;
  travelType: string;
  address: string | null;
  lat: number | null;
  lng: number | null;
};

function TextSelector({
  data,
  defaultValue,
  width,
}: {
  data: ComboboxData;
  defaultValue: string;
  width: string;
}) {
  return (
    <Select
      placeholder="Select mode of transport"
      size="xs"
      defaultValue={defaultValue}
      data={data}
      allowDeselect={false}
      styles={{ input: { width: width } }}
    />
  );
}
export function RequirementCard({
  defaultRequirement,
  onDelete,
  onChange,
}: {
  defaultRequirement: Requirement;
  onDelete: (id: string) => void;
  onChange: (req: Requirement) => void;
}) {
  const [autocomplete, setAutocomplete] =
    useState<google.maps.places.Autocomplete | null>(null);
  const [inputValue, setInputValue] = useState("");
  const [requirement, setRequirement] =
    useState<Requirement>(defaultRequirement);

  const onLoad = (autocompleteInstance: google.maps.places.Autocomplete) => {
    setAutocomplete(autocompleteInstance);
  };

  const onPlaceChanged = () => {
    if (autocomplete !== null) {
      const place = autocomplete.getPlace();
      const newAddress = place.formatted_address || null;
      setInputValue(newAddress || "");
      setRequirement((prev) => {
        const req = { ...prev };
        req.address = newAddress;
        if (place.geometry && place.geometry.location) {
          req.lat = place.geometry.location.lat();
          req.lng = place.geometry.location.lng();
        }
        return req;
      });
    }
  };
  return (
    <Card
      shadow="xs"
      padding="md"
      m="xs"
      radius="md"
      style={{ borderWidth: "3px", position: "relative" }}
    >
      <ActionIcon
        style={{
          position: "absolute",
          top: 10,
          right: 10,
          color: "gray",
          backgroundColor: "transparent",
        }}
        onClick={() => onDelete(requirement.id)}
      >
        <IconTrash size={12} />
      </ActionIcon>
      <Group gap="xs" align="center" p="10px">
        <Text size="sm" style={{ userSelect: "none" }}>
          Within
        </Text>
        <TextSelector
          defaultValue={requirement.duration.toString()}
          data={[
            { value: "5", label: "5 min" },
            { value: "10", label: "10 min" },
            { value: "15", label: "15 min" },
            { value: "20", label: "20 min" },
            { value: "30", label: "30 min" },
            { value: "60", label: "60 min" },
          ]}
          width="80px"
        />
        <TextSelector
          defaultValue={requirement.travelType}
          data={[
            { value: "walk", label: "walk" },
            { value: "cycle", label: "cycle" },
            { value: "drive", label: "drive" },
            { value: "public_transport", label: "public tranport" },
          ]}
          width="130px"
        />
      </Group>
      <Autocomplete
        onLoad={onLoad}
        onPlaceChanged={onPlaceChanged}
        options={{
          componentRestrictions: { country: "au" },
        }}
      >
        <TextInput
          placeholder="Enter address"
          value={inputValue}
          onChange={(event) => setInputValue(event.currentTarget.value)}
        />
      </Autocomplete>
    </Card>
  );
}
