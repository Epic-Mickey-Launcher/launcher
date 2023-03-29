let callbacks = []

export function Subscribe(name, cb) {
    callbacks.push({
        name: name,
        callback: cb
    })
}

export function Invoke(name, args) {
    let allcallbacks = callbacks.filter(e => e.name === name)

    allcallbacks.forEach(e => {
        e.callback(args)

        let index = allcallbacks.indexOf(e)

        allcallbacks.splice(index, 1)

    })
}