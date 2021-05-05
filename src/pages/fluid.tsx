

const FluidSimulation = () => {
  import('wasm/pkg').then((module) => console.log(module.fib(10)));
  return <div></div>;
};

export default FluidSimulation;
