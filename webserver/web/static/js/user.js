"use strict";

function _toConsumableArray(arr) { return _arrayWithoutHoles(arr) || _iterableToArray(arr) || _nonIterableSpread(); }

function _nonIterableSpread() { throw new TypeError("Invalid attempt to spread non-iterable instance"); }

function _iterableToArray(iter) { if (Symbol.iterator in Object(iter) || Object.prototype.toString.call(iter) === "[object Arguments]") return Array.from(iter); }

function _arrayWithoutHoles(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = new Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } }

(function () {
  function getCheckboxes() {
    return document.getElementsByName('paste-delete');
  }

  function buttonVisibilityCheck() {
    var button = document.getElementById('delete-button');

    if (button === null) {
      return;
    }

    if (_toConsumableArray(getCheckboxes()).some(function (x) {
      return x.checked;
    })) {
      button.classList.remove('is-hidden');
    } else {
      button.classList.add('is-hidden');
    }
  }

  function getPastesToDelete() {
    return _toConsumableArray(getCheckboxes()).filter(function (x) {
      return x.checked;
    }).map(function (x) {
      return x.dataset.pasteId;
    });
  }

  function addInput(form) {
    var input = document.createElement('input');
    input.type = 'hidden';
    input.name = 'ids';
    input.value = JSON.stringify(getPastesToDelete());
    form.appendChild(input);
  }

  (function () {
    getCheckboxes().forEach(function (x) {
      return x.addEventListener('change', buttonVisibilityCheck);
    });
    var form = document.getElementById('deletion_form');

    if (form !== null) {
      form.addEventListener('submit', function () {
        return addInput(form);
      });
    }
  })();

  function allCheckboxes(on) {
    document.querySelectorAll('input[type=checkbox]').forEach(function (e) {
      return e.checked = on;
    });
    buttonVisibilityCheck();
  }

  (function () {
    var selectAll = document.getElementById('select-all');

    if (selectAll !== null) {
      selectAll.addEventListener('click', function () {
        return allCheckboxes(true);
      });
    }

    var selectNone = document.getElementById('select-none');

    if (selectNone !== null) {
      selectNone.addEventListener('click', function () {
        return allCheckboxes(false);
      });
    }
  })();
})();
//# sourceMappingURL=user.js.map