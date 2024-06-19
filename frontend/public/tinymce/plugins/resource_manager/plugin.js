/**
 * plugin.js
 *
 * Copyright, Alberto Peripolli
 * Released under Creative Commons Attribution-NonCommercial 3.0 Unported License.
 *
 */

tinymce.PluginManager.add('resource_manager', function (editor) {
  const settings = {
    resource_manager_title: 'resource_manager',
    resource_manager_sort_by: 'name',
    resource_manager_descending: false,
    resource_manager_subfolder: '',
    resource_manager_crossdomain: false,
    language: 'en'
  }

  // From TinyMCE 6.0 the settings API has changed
  if (tinymce.majorVersion > 5) {
    // register settings
    editor.options.register('resource_manager_url', {
      processor: 'string',
      default: '/filemanager/'
    })
    editor.options.register('resource_manager_title', {
      processor: 'string',
      default: '资源管理器'
    })
    editor.options.register('resource_manager_sort_by', {
      processor: 'string',
      default: 'name'
    })
    editor.options.register('resource_manager_descending', {
      processor: 'boolean',
      default: false
    })
    editor.options.register('resource_manager_subfolder', {
      processor: 'string',
      default: ''
    })
    editor.options.register('resource_manager_crossdomain', {
      processor: 'boolean',
      default: false
    })
  }

  resolveSettings()

  function resolveSettings () {
    if (tinymce.majorVersion > 5) {
      settings.resource_manager_url = editor.options.get('resource_manager_url')
      settings.resource_manager_title = editor.options.get('resource_manager_title')
      settings.resource_manager_sort_by = editor.options.get('resource_manager_sort_by')
      settings.resource_manager_descending = editor.options.get('resource_manager_descending')
      settings.resource_manager_subfolder = editor.options.get('resource_manager_subfolder')
      settings.resource_manager_crossdomain = editor.options.get('resource_manager_crossdomain')
      settings.language = editor.options.get('language')
    } else {
      settings.resource_manager_url = editor.settings.resource_manager_url
      settings.resource_manager_title = editor.settings.resource_manager_title
      settings.resource_manager_sort_by = editor.settings.resource_manager_sort_by
      settings.resource_manager_descending = editor.settings.resource_manager_descending
      settings.resource_manager_subfolder = editor.settings.resource_manager_subfolder
      settings.resource_manager_crossdomain = editor.settings.resource_manager_crossdomain
      settings.language = editor.settings.language
    }
  }

  function resource_manager_onMessage (event) {
    if (settings.resource_manager_url.toLowerCase().indexOf(event.origin.toLowerCase()) === 0) {
      if (event.data.sender === 'responsivefilemanager') {
        tinymce.activeEditor.insertContent(event.data.html)
        tinymce.activeEditor.windowManager.close()

        // Remove event listener for a message from ResponsiveFilemanager
        if (window.removeEventListener) {
          window.removeEventListener('message', resource_manager_onMessage, false)
        } else {
          window.detachEvent('onmessage', resource_manager_onMessage)
        }
      }
    }
  }

  function openmanager () {
    resolveSettings()
    var width = window.innerWidth - 100
    var height = window.innerHeight - 200
    if (width > 1800) width = 1800
    if (height > 1200) height = 1200
    if (width > 600) {
      var width_reduce = (width - 20) % 138
      width = width - width_reduce + 10
    }

    editor.focus(true)
    var title = ''
    if (typeof settings.resource_manager_title !== 'undefined' && settings.resource_manager_title) {
      title = settings.resource_manager_title
    }
    var sort_by = ''
    if (typeof settings.resource_manager_sort_by !== 'undefined' && settings.resource_manager_sort_by) {
      sort_by = '&sort_by=' + settings.resource_manager_sort_by
    }
    var descending = 'false'
    if (typeof settings.resource_manager_descending !== 'undefined' && settings.resource_manager_descending) {
      descending = settings.resource_manager_descending
    }
    var fldr = ''
    if (typeof settings.resource_manager_subfolder !== 'undefined' && settings.resource_manager_subfolder) {
      fldr = '&fldr=' + settings.resource_manager_subfolder
    }
    var crossdomain = ''
    if (typeof settings.resource_manager_crossdomain !== 'undefined' && settings.resource_manager_crossdomain) {
      crossdomain = '&crossdomain=1'

      // Add handler for a message from ResponsiveFilemanager
      if (window.addEventListener) {
        window.addEventListener('message', resource_manager_onMessage, false)
      } else {
        window.attachEvent('onmessage', resource_manager_onMessage)
      }
    }

    // const fileUrl = settings.resource_manager_url + 'dialog.php?type=4&descending=' + descending + sort_by + fldr + crossdomain + '&lang=' + settings.language + '&akey=' + akey
    const fileUrl = settings.resource_manager_url

    if (tinymce.majorVersion < 5) {
      win = editor.windowManager.open({
        title: title,
        file: fileUrl,
        width: width,
        height: height,
        inline: 1,
        resizable: true,
        maximizable: true
      })
    } else {
      win = editor.windowManager.openUrl({
        title: title,
        url: fileUrl,
        width: width,
        height: height,
        inline: 1,
        resizable: true,
        maximizable: true
      })
    }
  }

  if (tinymce.majorVersion < 5) {
    editor.addButton('resource_manager', {
      icon: 'browse',
      tooltip: 'Insert file',
      shortcut: 'Ctrl+E',
      onClick: openmanager
    })

    editor.addShortcut('Ctrl+E', '', openmanager)

    editor.addMenuItem('resource_manager', {
      icon: 'browse',
      text: 'Insert file',
      shortcut: 'Ctrl+E',
      onClick: openmanager,
      context: 'insert'
    })
  } else {
    editor.ui.registry.addButton('resource_manager', {
      icon: 'browse',
      tooltip: 'Insert file',
      shortcut: 'Ctrl+E',
      onAction: openmanager
    })

    editor.addShortcut('Ctrl+E', '', openmanager)

    editor.ui.registry.addMenuItem('resource_manager', {
      icon: 'browse',
      text: 'Insert file',
      shortcut: 'Ctrl+E',
      onAction: openmanager,
      context: 'insert'
    })
  }
})
