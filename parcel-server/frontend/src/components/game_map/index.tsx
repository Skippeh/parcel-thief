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
import { getQpidAreas } from "../../services/game_data_service";
import { RefObject, useEffect, useRef, useState } from "react";
import { Area, QpidArea } from "../../api_types";
import Compass, { Tunnel as CompassTunnel } from "./compass";

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

const Wrapper = styled.div`
  position: relative;
  height: 100%;
`;

interface Props {
  area: Area;
}

interface InnerProps extends Props {
  mapControlsRef: RefObject<MapControlsImpl>;
}

const GameMap = (props: Props) => {
  const mapControlsRef = useRef<MapControlsImpl>(null);

  return (
    <Wrapper>
      <Canvas frameloop="demand">
        <MapRender {...props} mapControlsRef={mapControlsRef} />
      </Canvas>
      <CompassTunnel.Out />
    </Wrapper>
  );
};

const MapRender = ({ area, mapControlsRef }: InnerProps) => {
  const three = useThree();
  const planeTexture = useLoader(TextureLoader, Textures.get(area));
  planeTexture.anisotropy = Math.min(
    8,
    three.gl.capabilities.getMaxAnisotropy()
  );
  planeTexture.needsUpdate = true;
  const heightTexture = useLoader(TextureLoader, HeightTextures.get(area));

  const [qpidAreas, setQpidAreas] = useState<QpidArea[] | null | undefined>(
    undefined
  );

  useEffect(() => {
    if (qpidAreas == null) {
      (async () => {
        const response = await getQpidAreas();

        if (response.data != null) {
          setQpidAreas(response.data);
        } else {
          alert("Failed to get qpid areas: " + response.error);
        }
      })();
    }
  }, []);

  useEffect(() => {
    mapControlsRef.current.setAzimuthalAngle(0);
    mapControlsRef.current.setPolarAngle(0);
  }, [mapControlsRef, area]);

  return (
    <>
      <color attach="background" args={["black"]} />
      <ambientLight args={["white", 3]} />
      <PerspectiveCamera
        makeDefault
        position={[0, 0, 1000]}
        up={[0, 0, 1]}
        // rotate facing down
        rotation={[-Math.PI / 2, 0, 0]}
      />
      <MapControls
        ref={mapControlsRef}
        makeDefault
        maxPolarAngle={Math.PI / 2}
        minPolarAngle={-Math.PI / 2}
        enableRotate
        enableDamping
        minDistance={128}
        maxDistance={1024}
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
      {qpidAreas && <QpidIcons areas={qpidAreas} area={area} />}
      <Compass controlsRef={mapControlsRef} />
    </>
  );
};

export default GameMap;
