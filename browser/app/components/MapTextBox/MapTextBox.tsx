import type { MapTileResponse } from "~/client";
import type { Requirement } from "~/utils/requirementUtils";

const getColorForScore = (score: number) => {
  const red = Math.min(255, Math.floor((100 - score) * 2.55));
  const green = Math.min(255, Math.floor(score * 2.55));
  return `rgb(${red}, ${green}, 0)`;
};

export const MapTextBox = ({
  tile,
  requirements,
}: {
  tile: MapTileResponse | null;
  requirements: Requirement[];
}) => {
  return (
    <div
      style={{
        position: "absolute",
        top: "20px",
        right: "20px",
        backgroundColor: "rgba(255, 255, 255, 0.8)",
        padding: "0px 10px",
        borderRadius: "5px",
        boxShadow: "0 0 10px rgba(0,0,0,0.5)",
        zIndex: 1000,
        fontSize: "12px",
      }}
    >
      {tile ? (
        <>
          {/* <h3 style={{ textAlign: "center" }}>{tile.meanScore}%</h3> */}
          {tile.requirementScores.map((req) => {
            const requirement = requirements.find(
              (r) => r.id === req.requirementId,
            );
            if (requirement && requirement.location) {
              const address = requirement.location.address.split(",")[0];
              const scoreColor = getColorForScore(req.score);
              return (
                <p key={req.requirementId}>
                  <strong style={{ color: scoreColor }}>{req.score}%</strong>{" "}
                  {address}
                </p>
              );
            }
            return null;
          })}
        </>
      ) : (
        <h3 style={{ textAlign: "center" }}>No Tile Selected</h3>
      )}
    </div>
  );
};
