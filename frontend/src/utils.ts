import { DependencyList, useEffect } from "react";

export function useEffectOnce(
    action: () => void
) {
    useEffect(action, [])
}

/**
 * Shortcut function for running async function inside
 * useEffect hooks. Saves on boilerplate.
 *
 * @param action Async action to run in useEffect
 * @param dependencies Hook dependencies
 */
export function useEffectAsync(
    action: () => Promise<void>,
    dependencies: DependencyList = []
) {
    useEffect(() => {
        action()
            .then()
            .catch(console.error)
    }, dependencies)
}