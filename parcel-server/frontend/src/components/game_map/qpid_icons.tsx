import { Html } from "@react-three/drei";
import { Area, QpidArea } from "../../api_types";
import { styled } from "styled-components";
import IconPreppersShelter from "../../../../../assets/ds/icons/preppers.png";
import IconDeliveryBase from "../../../../../assets/ds/icons/deliveryBase.png";
import IconCrematory from "../../../../../assets/ds/icons/crematory.png";
import IconWindFarm from "../../../../../assets/ds/icons/windfarm.png";
import IconRainFarm from "../../../../../assets/ds/icons/rainfarm.png";
import IconWeatherStation from "../../../../../assets/ds/icons/weather_station.png";
import IconMamaFacility from "../../../../../assets/ds/icons/mama_facility.png";
import IconHeartmanFacility from "../../../../../assets/ds/icons/heartman_facility.png";
import IconRelayStation from "../../../../../assets/ds/icons/relay_station.png";
import IconNpcPostbox from "../../../../../assets/ds/icons/postbox_npc.png";

const IconWrapper = styled.div`
  transform: translateX(-50%);
  user-select: none;
  font-weight: 300;
  width: 300px;
  font-size: 12px;
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;

  & img {
    pointer-events: none;
  }

  & .icons {
    width: 25px;
    position: relative;

    & > img {
      width: 100%;
      position: absolute;
      left: 0;
      top: 0;

      &:first-child {
        background: rgba(0, 0, 0, 0.3);
        box-shadow: 0 0 17px #000;
        border-radius: 50%;
      }
    }
  }

  & .name {
    text-align: center;
    margin-top: 1rem;
    --shadow-color: rgba(44, 137, 231, 1);
    text-shadow: 0 0 20px var(--shadow-color), 0 0 20px var(--shadow-color),
      0 0 20px var(--shadow-color), 0 0 20px var(--shadow-color),
      0 0 20px var(--shadow-color), 0 0 5px #000, 0 0 5px #000;
  }
`;

interface Props {
  areas: QpidArea[];
  area: Area;
}

// Don't show these areas on the map
const IgnoreQpidIds = new Set([
  // UCA-41-011: Potential Chiral Relay Construction Site
  250,
  // Chiral Relay
  251,
]);

const QpidIcons = ({ areas, area }: Props) => {
  return (
    <>
      {areas
        .filter(
          (area2) =>
            area2.metadata.area === area &&
            (area2.metadata.constructionType == "deliveryBase" ||
              area2.metadata.constructionType == "preppersShelter") &&
            !IgnoreQpidIds.has(area2.qpidId)
        )
        .map((area) => {
          const position = convertCoordinates(
            area.metadata.location,
            area.metadata.area
          );
          return (
            <Html position={position}>
              <IconWrapper>
                <div className="icons">
                  <img className="icon" src={getQpidAreaIcon(area)} />
                </div>
                <span className="name">
                  <div className="background-blur" />
                  {area.names["en-us"]}
                </span>
              </IconWrapper>
            </Html>
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
 *
 * Note that some areas have different scaling due to different terrain size or max height.
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

const IconOverrides: Map<number, string> = new Map([
  // Incinerator West of Capital Knot City
  [103, IconCrematory],
  // Incinerator West of Lake Knot City
  [236, IconCrematory],
  // Wind Farm
  [106, IconWindFarm],
  // Timefall Farm
  [204, IconRainFarm],
  // Weather Station
  [227, IconWeatherStation],
  // Mama's Lab
  [233, IconMamaFacility],
  // Heartman's Lab
  [239, IconHeartmanFacility],
  // UCA-41-011: Potential Chiral Relay Construction Site
  [250, IconRelayStation],
  // Chiral Relay
  [251, IconRelayStation],
  // Various npc postboxes
  [1100, IconNpcPostbox],
  [1101, IconNpcPostbox],
  [1290, IconNpcPostbox],
  [1295, IconNpcPostbox],
  [1296, IconNpcPostbox],
  [1297, IconNpcPostbox],
]);

function getQpidAreaIcon(area: QpidArea): string {
  if (IconOverrides.has(area.qpidId)) {
    return IconOverrides.get(area.qpidId);
  }

  switch (area.metadata.constructionType) {
    case "deliveryBase":
      return IconDeliveryBase;
    case "preppersShelter":
      return IconPreppersShelter;
  }
}

export default QpidIcons;
