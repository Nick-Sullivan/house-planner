import React, { createContext, useContext, useEffect, useState } from "react";

type LeafletComponents = {
  MapContainer: any;
  Marker: any;
  Polygon: any;
  TileLayer: any;
};

const MapContext = createContext<LeafletComponents | null>(null);

export const MapProvider = ({ children }: { children: React.ReactNode }) => {
  // React leaflet fails when used for SSR, which is triggered by react-router
  // even if SSR is disabled, so it's lazy loaded here.
  const [leafletComponents, setLeafletComponents] =
    useState<LeafletComponents | null>(null);

  useEffect(() => {
    import("react-leaflet").then((module) => {
      setLeafletComponents({
        MapContainer: module.MapContainer,
        Marker: module.Marker,
        Polygon: module.Polygon,
        TileLayer: module.TileLayer,
      });
    });
  }, []);

  return (
    <MapContext.Provider value={leafletComponents}>
      {children}
    </MapContext.Provider>
  );
};

export const useMap = () => useContext(MapContext);
