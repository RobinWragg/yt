import { useState } from "react";
import "./yt.css";

function Yt() {
  const [count, setCount] = useState(0);

  return (
    <>
      <h1>This is 'yt'</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
      </div>
    </>
  );
}

export default Yt;
