# Application: #
    - Layers, TBC
    - Creates the Window

# Events: #
    - Keyboard events
    - Mouse events
    - Window events
    - Engine events

## Event Messaging ##
When an event occurs within the Window, the Window must create an Event object and send that event back to the application.

Instead of having Window hold an application pointer, we want Window to have a callback like a function pointer.

Application has a function called OnEvent which is called by Application. This function updates Application on Window events. This can be blocking if we only deal with one event at a time, or it can be non-blocking if we buffer events and deal with them serially.

Need an event dispatcher.
