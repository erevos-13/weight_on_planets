import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [weight, setWeight] = useState(null);
  const [planet, setPlanet] = useState(0);
  const [response, setResponse] = useState("");

   function handleChange(event) {
        setPlanet( event.target.value);
    }

    async function actionFn() {
       console.log({planet, weight})
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
       try {
         const res: string =  await invoke("find_weight_on_planet", {
             planetId: +planet,
             weightNumber: +weight
           })
           console.log(res)
           setResponse(res);
       }catch (e) {
           console.log(e)
       }


  }
  return (
    <div className="flex flex-col w-100 max-w-5xl m-auto p-20 h-screen justify-center align-middle">
        <div className="flex flex-col justify-center align-middle">
            <div className="text-5xl text-center p-5">Welcome to <br/><b>Weight On Planets</b></div>
            <div className="text-3xl text-center p-5">It is simple app for me to start learn Rust.</div>
            <form>
                <div className="flex flex-row justify-between align-middle">
                    <div className="flex flex-row justify-center align-middle">
                        <div className="form-container">
                            <label htmlFor="planets" className="p-2">Choose a car:</label>
                            <select onChange={handleChange} name="planets" id="planets">
                                <option selected >Select</option>
                                <option value={1}>Moon</option>
                                <option value={2}>Mercury</option>
                                <option value={3}>Venus</option>
                                <option value={4}>Mars</option>
                                <option value={5}>Jupyter</option>
                                <option value={6}>Neptune</option>
                                <option value={7}>Uranus</option>
                                <option value={8}>Saturn</option>
                            </select>
                        </div>

                    </div>
                    <div className="flex flex-col justify-center align-middle">
                        <input
                            id="planet-input"
                            onChange={(e) => setWeight(e.currentTarget.value)}
                            placeholder="Enter a your weight..."
                            className="w-100"
                        />
                        <button disabled={planet === 0 || !weight } type="button" onClick={() => actionFn()}>
                            Calculate
                        </button>
                    </div>
                </div>
            </form>
            <p>{response}</p>
        </div>

    </div>
  );
}

export default App;
