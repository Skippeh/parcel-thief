import styled from "styled-components";

const Wrapper = styled.div<{ $scale: number }>`
  transform: translateX(-50%);
  user-select: none;
  font-weight: 300;
  font-size: 12px;
  position: relative;
  pointer-events: ${(p) => (p.$scale <= 0.25 ? "none" : "auto")};

  & .icons {
    width: 25px;
    height: 25px;
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
    margin-top: -0.5rem;
    --shadow-color: rgba(44, 137, 231, 1);
    text-shadow: 0 0 20px var(--shadow-color), 0 0 20px var(--shadow-color),
      0 0 20px var(--shadow-color), 0 0 20px var(--shadow-color),
      0 0 20px var(--shadow-color), 0 0 5px #000, 0 0 5px #000;

    white-space: nowrap;
  }

  & .inner {
    transform: scale(${(p) => p.$scale});

    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;

    transition: transform 0.2s cubic-bezier(0, 1, 1, 1);
  }

  &:hover .inner {
    cursor: pointer;
    transform: scale(1.1);
  }
`;

interface Props {
  iconSrc: string;
  label?: string;
  importance: "low" | "high";
  cameraDistance: number;
}

const Icon = ({ iconSrc, label, importance, cameraDistance }: Props) => {
  function onClick() {
    console.log("clicked");
  }

  function getScale() {
    if (importance == "high") {
      return 1;
    } else if (importance == "low") {
      // scale from 1-0 between min and max
      const MinHeight = 200;
      const MaxHeight = 600;
      if (cameraDistance <= MinHeight) {
        return 1;
      } else if (cameraDistance >= MaxHeight) {
        return 0;
      } else {
        return 1 - (cameraDistance - MinHeight) / (MaxHeight - MinHeight);
      }
    }
  }

  return (
    <Wrapper $scale={getScale()}>
      <div className="inner" onClick={onClick}>
        <div className="icons">
          <img className="icon" src={iconSrc} />
        </div>
        {label && <span className="name">{label}</span>}
      </div>
    </Wrapper>
  );
};

export default Icon;
