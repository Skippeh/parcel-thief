import { ArrowCircleUp, Compass as CompassIcon } from "@phosphor-icons/react";
import { Html } from "@react-three/drei";
import { useFrame, useThree } from "@react-three/fiber";
import { RefObject, useEffect, useState } from "react";
import styled from "styled-components";
import { MapControls } from "three-stdlib";
import tunnel from "tunnel-rat";

export const Tunnel = tunnel();

const Wrapper = styled.div`
  position: absolute;
  right: 1rem;
  bottom: 1rem;
`;

interface Props {
  controlsRef: RefObject<MapControls>;
}

const Compass = ({ controlsRef }: Props) => {
  const [azimuthAngle, setAzimuthAngle] = useState(0);

  function resetAngles() {
    if (controlsRef.current != null) {
      controlsRef.current.setAzimuthalAngle(0);
      controlsRef.current.setPolarAngle(0);
    }
  }

  useFrame(() => {
    if (controlsRef.current == null) {
      return;
    }

    setAzimuthAngle(controlsRef.current.getAzimuthalAngle());
  });

  function getTransformStyle(): React.CSSProperties {
    return {
      transform: `rotateZ(${azimuthAngle}rad)`,
    };
  }

  return (
    <Html zIndexRange={[-1, 0]}>
      <Tunnel.In>
        <Wrapper>
          <div
            style={{ ...getTransformStyle(), width: "40px", height: "40px" }}
          >
            <a onClick={resetAngles} title="Reset rotation">
              <ArrowCircleUp size={40} />
            </a>
          </div>
        </Wrapper>
      </Tunnel.In>
    </Html>
  );
};

export default Compass;
