import { ActionIcon, TextInput } from "@mantine/core";
import { IconSearch, IconX } from "@tabler/icons-react";
import { debounce } from "radash";
import { useMemo, useState } from "react";
import { useSearchParams } from "react-router";

export function SearchBox({
  searchParam,
  onChange,
}: {
  searchParam: string;
  onChange: (value: string) => void;
}) {
  const [searchParams, _] = useSearchParams();
  const [localSearchState, setLocalSearchState] = useState(
    () => searchParams.get(searchParam) ?? ""
  );
  const debounceNotify = useMemo(
    () =>
      debounce({ delay: 200 }, (value: string) => {
        onChange(value);
      }),
    [onChange]
  );
  const onLocalChange = (value: string) => {
    setLocalSearchState(value);
    debounceNotify(value);
  };
  return (
    <TextInput
      px="xs"
      pt={3}
      leftSection={<IconSearch size={16} />}
      leftSectionPointerEvents="none"
      rightSection={
        <ActionIcon
          variant="transparent"
          onClick={() => onLocalChange("")}
          style={{ display: localSearchState ? "block" : "none" }}
        >
          <IconX size={16} color="var(--mantine-color-gray-5)" />
        </ActionIcon>
      }
      value={localSearchState}
      onChange={(event) => onLocalChange(event.currentTarget.value)}
    />
  );
}
