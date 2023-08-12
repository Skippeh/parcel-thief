import { Html } from "@react-three/drei";
import { Area, QpidArea } from "../../api_types";

interface Props {
  areas: QpidArea[];
  area: Area;
}

const QpidIcons = ({ areas, area }: Props) => {
  return (
    <>
      {areas
        .filter(
          (area2) =>
            area2.metadata.area === area &&
            (area2.metadata.constructionType == "deliveryBase" ||
              area2.metadata.constructionType == "preppersShelter")
        )
        .map((area) => {
          const position = convertCoordinates(
            area.metadata.location,
            area.metadata.area
          );
          return (
            <mesh key={area.qpidId} position={position}>
              <boxGeometry args={[10, 10, 10]} />
              <meshStandardMaterial color={"red"} />
              <Html>
                <p>{area.names["en-us"]}</p>
              </Html>
            </mesh>
          );
        })}
    </>
  );
};

/**
 * Converts xyz from game coordinates to coordinates compatible with the three.js plane/terrain scale.
 *
 * Game coordinates range from 0-7168 on the XYZ axis.
 *
 * This function scales the values to 0-1024.
 * @returns
 */
function convertCoordinates(
  [x, y, z]: [number, number, number],
  area: Area
): [number, number, number] {
  let xyScale = 7168;
  let zScale = xyScale;

  // area02 / central has different Z scaling than the other levels, not quite sure what it is yet
  if (area === "area02") {
    zScale /= 4.5;
  }

  // area04 / west is half the size of the other levels
  if (area === "area04") {
    xyScale /= 2;
  }

  return [(x / xyScale) * 1024, (y / xyScale) * 1024, (z / zScale) * 128];
}

export default QpidIcons;
