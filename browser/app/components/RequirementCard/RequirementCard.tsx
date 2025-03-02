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
import * as h3 from "h3-js";
import { set } from "radash";
import { useEffect, useState } from "react";
import { TravelMode } from "~/client";
import { h3IndexLevel } from "~/utils/constants";
import type { Requirement } from "~/utils/requirementUtils";

function TextSelector({
  data,
  defaultValue,
  width,
  onChange,
}: {
  data: ComboboxData;
  defaultValue: string;
  width: string;
  onChange: (value: string | null) => void;
}) {
  return (
    <Select
      placeholder="Select mode of transport"
      size="xs"
      defaultValue={defaultValue}
      data={data}
      allowDeselect={false}
      styles={{ input: { width: width } }}
      onChange={onChange}
    />
  );
}

export function RequirementCard({
  requirement,
  onDelete,
  onChange,
}: {
  requirement: Requirement;
  onDelete: (id: string) => void;
  onChange: (req: Requirement) => void;
}) {
  const [autocomplete, setAutocomplete] =
    useState<google.maps.places.Autocomplete | null>(null);
  const [addressText, setAddressText] = useState(
    requirement.location?.address || "",
  );
  const onLoad = (autocompleteInstance: google.maps.places.Autocomplete) => {
    setAutocomplete(autocompleteInstance);
  };
  const onPlaceChanged = () => {
    if (autocomplete === null) {
      return;
    }
    const place = autocomplete.getPlace();
    const newAddress = place.formatted_address || null;
    const isValid = newAddress && place.geometry && place.geometry.location;
    setAddressText(newAddress || "");
    let location = null;
    if (isValid) {
      const geoLoc = place.geometry!.location!;
      const h3Index = h3.latLngToCell(geoLoc.lat(), geoLoc.lng(), h3IndexLevel);
      location = {
        address: newAddress,
        h3Index: h3Index,
        lat: geoLoc.lat(),
        lng: geoLoc.lng(),
      };
    }
    onChange({ ...requirement, location: location });
  };
  const onDurationChanged = (value: number) => {
    onChange({ ...requirement, duration: value });
  };
  const onTravelTypeChanged = (value: TravelMode) => {
    onChange({ ...requirement, travelType: value });
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
          onChange={(value) => onDurationChanged(parseInt(value!, 10))}
        />
        <TextSelector
          defaultValue={requirement.travelType}
          data={[
            { value: TravelMode.Walking, label: "walk" },
            { value: TravelMode.Bicycling, label: "cycle" },
            { value: TravelMode.Driving, label: "drive" },
            { value: TravelMode.PublicTransport, label: "public tranport" },
          ]}
          width="130px"
          onChange={(value) => onTravelTypeChanged(value! as TravelMode)}
        />
      </Group>
      <Autocomplete
        onLoad={onLoad}
        onPlaceChanged={onPlaceChanged}
        options={{
          componentRestrictions: { country: "au" },
          bounds: new google.maps.LatLngBounds(
            new google.maps.LatLng(-38.0625, 129.0019),
            new google.maps.LatLng(-25.9966, 141.0021),
          ),
          strictBounds: true,
        }}
      >
        <TextInput
          placeholder="Enter address"
          value={addressText}
          onChange={(event) => setAddressText(event.currentTarget.value)}
        />
      </Autocomplete>
    </Card>
  );
}
