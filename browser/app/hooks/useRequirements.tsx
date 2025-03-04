import { useEffect, useMemo, useRef, useState } from "react";
import type { MapResponse } from "~/client";
import { mapApi } from "~/utils/apiClient";
import {
  isCompletedRequirement,
  requirementToRequest,
  type Requirement,
} from "~/utils/requirementUtils";

export function useRequirements(initialCityCode: string = "Adelaide") {
  const [requirements, setRequirements] = useState<Requirement[]>([]);
  const [map, setMap] = useState<MapResponse | null>(null);
  const prevIdsRef = useRef<string[]>([]);

  const completedRequirementIds = useMemo(() => {
    // Can't just use filter here because it triggers even if the content is the same.
    const completed = requirements.filter(isCompletedRequirement);
    const currentIds = completed.map((r) => r.id);
    const prevSorted = [...prevIdsRef.current].sort().join(",");
    const currentSorted = [...currentIds].sort().join(",");
    if (prevSorted === currentSorted) {
      return prevIdsRef.current;
    }
    prevIdsRef.current = currentIds;
    return currentIds;
  }, [requirements]);
  const onRequirementChange = async (req: Requirement) => {
    console.log("onRequirementChange", req);
    if (isCompletedRequirement(req)) {
      const reqRequest = requirementToRequest(req);
      console.log("postRequirement", reqRequest);
      await mapApi.postRequirement({ requirementRequest: reqRequest });
    }
    setRequirements((prevRequirements) => {
      const existingRequirement = prevRequirements.find((r) => r.id === req.id);
      if (existingRequirement) {
        return prevRequirements.map((r) => (r.id === req.id ? req : r));
      } else {
        return [...prevRequirements, req];
      }
    });
  };
  const onRequirementDelete = (id: string) => {
    const existingRequirement = requirements.find((r) => r.id === id);
    if (!existingRequirement) {
      return;
    }
    setRequirements((prevRequirements) =>
      prevRequirements.filter((r) => r.id !== id),
    );
    // TODO: Delete from backend
  };
  useEffect(() => {
    console.log("getting map");
    mapApi
      .getMap({
        mapRequest: {
          cityCode: initialCityCode,
          requirementIds: completedRequirementIds,
        },
      })
      .then((mapResponse) => {
        setMap(mapResponse);
      });
  }, [completedRequirementIds, initialCityCode]);
  return {
    requirements,
    map,
    onRequirementChange,
    onRequirementDelete,
  };
}
