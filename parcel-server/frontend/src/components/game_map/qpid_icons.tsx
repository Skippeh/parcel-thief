import { Html } from "@react-three/drei";
import { Area, QpidArea, QpidObject, QpidObjectType } from "../../api_types";
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
import IconMotorbike from "../../../../../assets/ds/icons/motorbike.png";
import IconTruck from "../../../../../assets/ds/icons/truck.png";
import IconLadder from "../../../../../assets/ds/icons/ladder.png";
import IconSign from "../../../../../assets/ds/icons/signboard.png";
import IconBridge from "../../../../../assets/ds/icons/bridge.png";
import IconCargoCatapult from "../../../../../assets/ds/icons/cargo_catapult_online.png";
import IconCargoCatapultPod from "../../../../../assets/ds/icons/cargo_catapult_pod.png";
import IconOnlinePostBox from "../../../../../assets/ds/icons/postbox_online.png";
import IconJumpRamp from "../../../../../assets/ds/icons/ramp.png";
import IconRope from "../../../../../assets/ds/icons/rope.png";
import IconRainShelter from "../../../../../assets/ds/icons/shelter.png";
import IconWatchtower from "../../../../../assets/ds/icons/watchtower.png";
import IconZipline from "../../../../../assets/ds/icons/zipline.png";
import IconGenerator from "../../../../../assets/ds/icons/generator.png";
import IconChiralBridge from "../../../../../assets/ds/icons/chiral_bridge.png";
import IconSafehouse from "../../../../../assets/ds/icons/safehouse.png";
import IconQuestionMark from "../../../../../assets/ds/icons/question_mark.png";
import Icon from "./icon";

interface Props {
  areas: QpidArea[];
  area: Area;
  objects: QpidObject[];
}

// Don't show these areas on the map
const IgnoreQpidIds = new Set([
  // UCA-41-011: Potential Chiral Relay Construction Site
  250,
  // Chiral Relay
  251,
]);

const IgnoreObjectTypes = new Set<QpidObjectType>([
  "peeMushroom",
  "restingStone",
  "sign",
]);

const QpidIcons = ({ areas, objects, area }: Props) => {
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
            <Html key={area.qpidId} position={position}>
              <Icon
                iconSrc={getQpidAreaIcon(area)}
                label={area.names["en-us"]}
              />
            </Html>
          );
        })
        .concat(
          objects
            .filter((o) => !IgnoreObjectTypes.has(o.objectType))
            .map((object) => {
              const position = convertCoordinates(object.location, area);

              return (
                <Html key={object.id} position={position}>
                  <div
                    title={
                      (object.objectType != "unknown" &&
                      object.objectType != null
                        ? object.objectType
                        : JSON.stringify(object.unknownType)) +
                      ` (${object.creator.name})`
                    }
                  >
                    <Icon iconSrc={getQpidObjectIcon(object.objectType)} />
                  </div>
                </Html>
              );
            })
        )}
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

const AreaIconOverrides: Map<number, string> = new Map([
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

const ObjectIcons: Map<QpidObjectType, string> = new Map([
  ["motorbike", IconMotorbike],
  ["truck", IconTruck],
  ["ladder", IconLadder],
  ["sign", IconSign],
  ["bridge", IconBridge],
  ["cargoCatapult", IconCargoCatapult],
  ["cargoCatapultPod", IconCargoCatapultPod],
  ["postbox", IconOnlinePostBox],
  ["jumpRamp", IconJumpRamp],
  ["climbingAnchor", IconRope],
  ["timefallShelter", IconRainShelter],
  ["watchtower", IconWatchtower],
  ["zipline", IconZipline],
  ["powerGenerator", IconGenerator],
  ["chiralBridge", IconChiralBridge],
  ["safehouse", IconSafehouse],
  // no icon for peeMushroom
  // no icon for restingStone
]);

function getQpidAreaIcon(area: QpidArea): string {
  if (AreaIconOverrides.has(area.qpidId)) {
    return AreaIconOverrides.get(area.qpidId);
  }

  switch (area.metadata.constructionType) {
    case "deliveryBase":
      return IconDeliveryBase;
    case "preppersShelter":
      return IconPreppersShelter;
  }
}

function getQpidObjectIcon(objectType: QpidObjectType): string {
  if (ObjectIcons.has(objectType)) {
    return ObjectIcons.get(objectType);
  }

  return IconQuestionMark;
}

export default QpidIcons;
