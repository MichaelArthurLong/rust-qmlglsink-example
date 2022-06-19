import QtQuick 2.15
import QtQuick.Controls 2.15
import org.freedesktop.gstreamer.GLVideoItem 1.0
import RustGstPipeline 1.0

ApplicationWindow {
    id: root
    visible: true
    width: 640
    height: 480

    GstGLVideoItem {
	id: videoItem
	objectName: "videoItem"
	anchors.fill: parent
    }

    Button {
	id: button
	text: "Play"
	onClicked: {
	    console.log(gstPipeline)
	    gstPipeline.play()
	}
    }
}
