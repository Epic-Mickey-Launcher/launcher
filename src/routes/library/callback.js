let callbacks = []

export function Subscribe(name, cb, persistent) {
    callbacks.push({
        name: name,
        persistent: persistent,
        callback: cb
    })
}

export function Invoke(name, args) {
    let allcallbacks = callbacks.filter(e => e.name === name)

    allcallbacks.forEach(e => {
        e.callback(args)

        if (e.persistent === false || e.persistent === undefined) {

            let index = callbacks.indexOf(e)

            callbacks.splice(index, 1)
        }
    })
}