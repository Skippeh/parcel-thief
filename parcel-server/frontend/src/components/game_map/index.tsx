import { Html, MapControls, PerspectiveCamera } from "@react-three/drei";
import { Canvas, useFrame, useLoader, useThree } from "@react-three/fiber";
import { styled } from "styled-components";
import area01Texture from "../../../../../assets/ds/levels/area01/area01.jpg";
import area02Texture from "../../../../../assets/ds/levels/area02/area02.jpg";
import area04Texture from "../../../../../assets/ds/levels/area04/area04.jpg";
import area01HeightTexture from "../../../../../assets/ds/levels/area01/area01_height_lores.jpg";
import area02HeightTexture from "../../../../../assets/ds/levels/area02/area02_height_lores.jpg";
import area04HeightTexture from "../../../../../assets/ds/levels/area04/area04_height_lores.jpg";
import { TextureLoader } from "three";
import { MapControls as MapControlsImpl } from "three-stdlib";
import QpidIcons from "./qpid_icons";
import {
  QpidAreaWithChildren,
  getQpidAreas,
  organizeQpidAreas,
} from "../../services/game_data_service";
import { RefObject, useEffect, useRef, useState } from "react";
import { Area, Baggage, QpidArea, QpidObject } from "../../api_types";
import Compass, { Tunnel as CompassTunnel } from "./compass";
import { getQpidObjects } from "../../services/qpid_objects_service";
import { getBaggages } from "../../services/baggages_service";

const Textures: Map<Area, string> = new Map([
  ["area01", area01Texture],
  ["area02", area02Texture],
  ["area04", area04Texture],
]);

const HeightTextures: Map<Area, string> = new Map<Area, string>(
  new Map([
    ["area01", area01HeightTexture],
    ["area02", area02HeightTexture],
    ["area04", area04HeightTexture],
  ])
);

export const MaxCameraDistance = 1024;

const Wrapper = styled.div`
  position: relative;
  height: 100%;
`;

interface Props {
  area: Area;
}

const GameMap = (props: Props) => {
  return (
    <Wrapper>
      <Canvas frameloop="demand">
        <MapRender {...props} />
      </Canvas>
      <CompassTunnel.Out />
    </Wrapper>
  );
};

const MapRender = ({ area }: Props) => {
  const mapControlsRef = useRef<MapControlsImpl>(null);
  const three = useThree();
  const planeTexture = useLoader(TextureLoader, Textures.get(area));
  planeTexture.anisotropy = Math.min(
    8,
    three.gl.capabilities.getMaxAnisotropy()
  );
  planeTexture.needsUpdate = true;
  const heightTexture = useLoader(TextureLoader, HeightTextures.get(area));

  const [qpidAreas, setQpidAreas] = useState<
    QpidAreaWithChildren[] | null | undefined
  >(undefined);
  const [qpidObjects, setQpidObjects] = useState<
    QpidObject[] | null | undefined
  >(undefined);
  const [baggages, setBaggages] = useState<Baggage[] | null | undefined>(
    undefined
  );

  useEffect(() => {
    (async () => {
      setQpidAreas(undefined);
      setQpidObjects(undefined);
      setBaggages(undefined);

      const [areasResponseTask, objectsResponseTask, baggagesResponseTask] = [
        getQpidAreas(),
        getQpidObjects(area),
        getBaggages(area),
      ];

      const [areasResponse, objectsResponse, baggagesResponse] = [
        await areasResponseTask,
        await objectsResponseTask,
        await baggagesResponseTask,
      ];

      if (
        areasResponse.error != null ||
        objectsResponse.error != null ||
        baggagesResponse.error != null
      ) {
        alert(
          "Failed to get map data." +
            "\nareas error: " +
            areasResponse.error +
            "\nobjects error: " +
            objectsResponse.error +
            "\nbaggages error: " +
            baggagesResponse.error
        );
        setQpidAreas(null);
        setQpidObjects(null);
        setBaggages(null);
        return;
      }

      let [qpidAreas, objects, baggages] = organizeQpidAreas(
        areasResponse.data,
        objectsResponse.data,
        baggagesResponse.data
      );

      setQpidAreas(qpidAreas);
      setQpidObjects(objects);
      setBaggages(baggages);
    })();

    if (mapControlsRef.current != null) {
      mapControlsRef.current.setAzimuthalAngle(0);
      mapControlsRef.current.setPolarAngle(0);
    }
  }, [area]);

  return (
    <>
      <color attach="background" args={["black"]} />
      <ambientLight args={["white", 3]} />
      <PerspectiveCamera
        makeDefault
        position={[0, 0, MaxCameraDistance]}
        up={[0, 0, 1]}
        // rotate facing down
        rotation={[-Math.PI / 2, 0, 0]}
      />
      <MapControls
        ref={mapControlsRef}
        makeDefault
        maxPolarAngle={Math.PI / 2 - Math.PI / 8}
        minPolarAngle={-Math.PI / 2}
        minDistance={200}
        maxDistance={MaxCameraDistance}
        zoomSpeed={2}
      />
      <mesh position={[0, 0, 0]}>
        <planeGeometry args={[1024, 1024, 256, 256]} />
        <meshStandardMaterial
          map={planeTexture}
          displacementMap={heightTexture}
          displacementScale={128}
        />
      </mesh>
      {qpidAreas && qpidObjects && baggages && (
        <QpidIcons
          areas={qpidAreas}
          objects={qpidObjects}
          baggages={baggages}
          area={area}
        />
      )}
      <Compass controlsRef={mapControlsRef} />
    </>
  );
};

export default GameMap;
