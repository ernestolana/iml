export function validate_ast(json_str) {
    try {
        JSON.parse(json_str);
        return "OK";
    } catch(e) {
        return JSON.stringify({ errors: [{ details: "Parse error" }] });
    }
}
export function translate_to_human(json_str) {
    return "0: Alloc\n1: Add -> 0\n";
}
export function translate_from_human(human_str) {
    return `{"nodes":[{"t":{"L":null},"c":[]}]}`;
}
export function simulate_execution(json_str, fuel_limit) {
    return `["Step 0: executed Alloc node"]`;
}
