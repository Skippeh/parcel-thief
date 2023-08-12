import { MapControls, PerspectiveCamera } from "@react-three/drei";
import { Canvas, useLoader, useThree } from "@react-three/fiber";
import { useRef } from "react";
import { styled } from "styled-components";
import area01Texture from "../../../../../assets/ds/levels/area01/area01.jpg";
import area02Texture from "../../../../../assets/ds/levels/area02/area02.jpg";
import area04Texture from "../../../../../assets/ds/levels/area04/area04.jpg";
import area01HeightTexture from "../../../../../assets/ds/levels/area01/area01_height_lores.jpg";
import area02HeightTexture from "../../../../../assets/ds/levels/area02/area02_height_lores.jpg";
import area04HeightTexture from "../../../../../assets/ds/levels/area04/area04_height_lores.jpg";
import { TextureLoader } from "three";
import THREE = require("three");

export enum Area {
  East = "area01",
  Central = "area02",
  West = "area04",
}

const Textures: Map<Area, string> = new Map([
  [Area.East, area01Texture],
  [Area.Central, area02Texture],
  [Area.West, area04Texture],
]);

const HeightTextures: Map<Area, string> = new Map<Area, string>(
  new Map([
    [Area.East, area01HeightTexture],
    [Area.Central, area02HeightTexture],
    [Area.West, area04HeightTexture],
  ])
);

const Wrapper = styled.div`
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
    </Wrapper>
  );
};

const MapRender = ({ area }: Props) => {
  const three = useThree();
  const planeRef = useRef<THREE.PlaneGeometry>(null);
  const planeTexture = useLoader(TextureLoader, Textures.get(area));
  planeTexture.anisotropy = Math.min(
    8,
    three.gl.capabilities.getMaxAnisotropy()
  );
  planeTexture.needsUpdate = true;
  const heightTexture = useLoader(TextureLoader, HeightTextures.get(area));

  return (
    <>
      <color attach="background" args={["black"]} />
      <ambientLight args={["white", 3]} />
      <PerspectiveCamera makeDefault position={[0, 0, 1000]} />
      <MapControls
        screenSpacePanning
        makeDefault
        maxPolarAngle={Math.PI / 1.1}
        minPolarAngle={Math.PI / 2}
        maxAzimuthAngle={Math.PI * 2}
        minAzimuthAngle={-Math.PI * 2}
        enableRotate
        enableDamping
        mouseButtons={{
          LEFT: THREE.MOUSE.PAN,
          MIDDLE: THREE.MOUSE.DOLLY,
        }}
        minDistance={125}
        maxDistance={1000}
        zoomSpeed={2}
      />
      <mesh position={[0, 0, 0]}>
        <planeGeometry args={[1024, 1024, 128, 128]} ref={planeRef} />
        <meshStandardMaterial
          map={planeTexture}
          displacementMap={heightTexture}
          displacementScale={100}
        />
      </mesh>
    </>
  );
};

export default GameMap;
