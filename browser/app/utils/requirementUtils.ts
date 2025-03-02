import { TravelMode, type RequirementRequest } from "~/client";

export type Requirement = {
  id: string;
  duration: number;
  travelType: TravelMode;
  location: RequirementLocation | null;
};

export type RequirementLocation = {
  address: string;
  h3Index: string;
  lat: number;
  lng: number;
};

export function isCompletedRequirement(req: Requirement): boolean {
  return req.location !== null;
}

export function requirementToRequest(req: Requirement): RequirementRequest {
  return {
    cityCode: "Adelaide",
    requirementId: req.id,
    toleratedDuration: req.duration * 60,
    travelMode: req.travelType,
    locations: [
      {
        id: 0,
        address: req.location!.address,
        h3Index: req.location!.h3Index,
        lat: req.location!.lat,
        lng: req.location!.lng,
      },
    ],
  };
}
