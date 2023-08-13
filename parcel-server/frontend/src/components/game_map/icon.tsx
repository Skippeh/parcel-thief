import styled from "styled-components";

const Wrapper = styled.div`
  transform: translateX(-50%);
  user-select: none;
  font-weight: 300;
  font-size: 12px;
  position: relative;

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

    white-space: nowrap;
  }

  & .inner {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;

    transition: transform 0.1s ease-out;
  }

  &:hover .inner {
    cursor: pointer;
    transform: scale(1.1);
  }
`;

interface Props {
  iconSrc: string;
  label?: string;
}

const Icon = ({ iconSrc, label }: Props) => {
  function onClick() {
    console.log("clicked");
  }

  return (
    <Wrapper>
      <div className="inner" onClick={onClick}>
        <div className="icons">
          <img className="icon" src={iconSrc} />
        </div>
        <span className="name">{label}</span>
      </div>
    </Wrapper>
  );
};

export default Icon;
