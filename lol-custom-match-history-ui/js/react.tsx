function nonNull(val, fallback) { return Boolean(val) ? val : fallback };

function DOMparseChildren(children) {
    return children.map(child => {
        if (typeof child === 'string') {
            return document.createTextNode(child);
        }
        return child;
    })
}

function DOMparseNode(element, properties, children) {
    const el = document.createElement(element);
    Object.keys(nonNull(properties, {})).forEach(key => {
        el[key] = properties[key];
    })
    DOMparseChildren(children).forEach(child => {
        el.appendChild(child);
    });
    return el;
}

function DOMcreateElement(element, properties, ...children) {
    if (typeof element === 'function') {
        return element({
            ...nonNull(properties, {}),
            children
        });
    }
    return DOMparseNode(element, properties, children);
}