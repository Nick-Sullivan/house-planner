import React, { useEffect } from "react";
import type { Requirement } from "~/utils/requirementUtils";
import LoadingSpinner from "../LoadingSpinner/LoadingSpinner";
import { useMap } from "../MapContext/MapContext";

const RequirementsMap = ({
  mapRef,
  requirements,
  selectedRequirement,
}: {
  mapRef: React.RefObject<google.maps.Map | null>;
  requirements: Requirement[];
  selectedRequirement: Requirement | null;
}) => {
  const leafletComponents = useMap();
  useEffect(() => {
    if (selectedRequirement?.location && mapRef.current) {
      mapRef.current.panTo({
        lat: selectedRequirement.location.lat!,
        lng: selectedRequirement.location.lng!,
      });
    }
  }, [selectedRequirement]);

  if (!leafletComponents) {
    return <LoadingSpinner />;
  }

  return (
    <>
      {requirements.map(
        (req: Requirement) =>
          req.location && (
            <leafletComponents.Marker
              key={req.id}
              position={[req.location.lat, req.location.lng]}
              opacity={selectedRequirement?.id === req.id ? 1 : 0.3}
            />
          ),
      )}
    </>
  );
};

export default RequirementsMap;
