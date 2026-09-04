export const presets = {
    robotic_trajectory: {
        name: "Robotic Trajectory Task",
        human: "0: Alloc\n1: Var(robot_arm)\n2: Add -> 0, 1\n",
        ast: JSON.stringify({
            nodes: [
                { t: { L: null }, c: [], r: "Allocate memory for trajectory", o: "mem1" },
                { t: { V: "robot_arm" }, c: [], r: "Robot arm state" },
                { t: { A: null }, c: [0, 1], r: "Combine allocation and state" }
            ]
        }, null, 2)
    },
    quantum_matrix: {
        name: "Quantum State Matrix",
        human: "0: QuantumState(len=4)\n1: AlgebraicMatrix(4x4)\n2: Mul -> 0, 1\n3: Drop -> 2\n",
        ast: JSON.stringify({
            nodes: [
                { t: { Q: [0.707, 0, 0, 0.707] }, c: [], r: "Bell state" },
                { t: { B: [1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0], rows: 4, cols: 4 }, c: [], r: "Identity transform" },
                { t: { M: null }, c: [0, 1], r: "Apply transformation" },
                { t: { D: null }, c: [2], r: "Deallocate state" }
            ]
        }, null, 2)
    },
    infinite_loop: {
        name: "Infinite Loop Trap",
        human: "0: Add -> 0\n",
        ast: JSON.stringify({
            nodes: [
                { t: { A: null }, c: [0], r: "Self-referencing infinite loop" }
            ]
        }, null, 2)
    }
};
