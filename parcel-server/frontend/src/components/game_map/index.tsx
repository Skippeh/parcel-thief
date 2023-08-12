import { MapControls, PerspectiveCamera } from "@react-three/drei";
import { Canvas, useLoader, useThree } from "@react-three/fiber";
import { useRef } from "react";
import { styled } from "styled-components";
import area01Texture from "../../../../../assets/ds/levels/area01/area01.jpg";
import area02Texture from "../../../../../assets/ds/levels/area02/area02.jpg";
import area04Texture from "../../../../../assets/ds/levels/area04/area04.jpg";
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

const Wrapper = styled.div`
  height: 100%;
`;

interface Props {
  area: Area;
}

const GameMap = ({ area }: Props) => {
  const planeRef = useRef<THREE.PlaneGeometry>(null);
  const planeTexture = useLoader(TextureLoader, Textures.get(area));
  planeTexture.anisotropy = 16;
  planeTexture.needsUpdate = true;

  return (
    <Wrapper>
      <Canvas frameloop="demand">
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
            MIDDLE: THREE.MOUSE.MIDDLE,
          }}
          minDistance={100}
          maxDistance={1000}
          zoomSpeed={2}
        />
        <mesh position={[0, 0, 0]}>
          <planeGeometry args={[1024, 1024, 128, 128]} ref={planeRef} />
          <meshStandardMaterial map={planeTexture} />
        </mesh>
      </Canvas>
    </Wrapper>
  );
};

export default GameMap;
