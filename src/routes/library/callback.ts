interface Callback {
  name: string,
  persistent: boolean,
  callback: any
}

let callbacks: Callback[] = []

export function Subscribe(name: string, cb: any, persistent: boolean) {
  let callback: Callback = {
    name: name,
    persistent: persistent,
    callback: cb
  }
  callbacks.push(callback)
}

export function Invoke(name: string, args: any) {
  let allCallbacks = callbacks.filter(e => e.name === name)
  allCallbacks.forEach(e => {
    e.callback(args)
    if (e.persistent === false || e.persistent === undefined) {
      let index = callbacks.indexOf(e)
      callbacks.splice(index, 1)
    }
  })
}
