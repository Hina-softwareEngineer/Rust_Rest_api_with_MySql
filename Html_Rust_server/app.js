$(document).ready(function () {
  let name = document.querySelector("#name");
  let email = document.querySelector("#email");
  let age = document.querySelector("#age");

  let updateRecord = document.querySelector(".edit-form");
  let inputs = document.querySelector(".edit-f");
  updateRecord.style.display = "none";
  let sid = 0;

  $("#deleted").css("display", "none");
  $("#inserted").css("display", "none");
  $("#updated").css("display", "none");

  //----------------------get data-----------------
  async function getData() {
    let response = await fetch("https://rocket-restapi-crud.herokuapp.com");
    let data = await response.json();
    return data;
  }

  getData()
    .then((data) => {
      data.forEach((d) => {
        addSingleData(d, d.sid);
      });
    })
    .catch((e) => alert("Error, Try again."));

  // ----------------------------Add Data Code--------------------------
  $("#submit").click(function (e) {
    e.preventDefault();

    if (name.value == "" || email.value == "" || age.value == "") {
      alert("Please fill the fields first");
    } else {
      var arr = {
        sid: Math.floor(Math.random() * 1000 + 1),
        name: name.value,
        email: email.value,
        age: age.value,
      };

      $.post(
        "https://rocket-restapi-crud.herokuapp.com/add",
        JSON.stringify(arr)
      )
        .then(function (d) {
          addSingleData(arr, d.id);
          name.value = "";
          email.value = "";
          age.value = "";

          $("#inserted").text("Successfully saved.");
          $("#inserted").show();
          $("#inserted").css("background-color", "green");
          setTimeout(function () {
            $("#inserted").hide();
          }, 3000);
        })
        .catch(function (e) {
          $("#inserted").text("Failed to Save.");
          $("#inserted").show();
          $("#inserted").css("background-color", "red");
          setTimeout(function () {
            $("#inserted").hide();
          }, 3000);
        });
    }
  });

  // ---------------------------Edit Code ------------------------------
  $(".ul").on("click", "li", ".edit", function (e) {
    e.stopPropagation();
    e.preventDefault();
    sid = $(this).data("id");
    let name = $(this).children(".name").text();
    let email = $(this).children(".email").text();
    let age = $(this).children(".age").text();

    updateRecord.style.display = "block";
    $("html, body").animate(
      {
        scrollTop: $(".edit-form").offset().top,
      },
      1000
    );

    inputs[0].value = name;
    inputs[1].value = email;
    inputs[2].value = age;
  });

  $(".ok").on("click", function (e) {
    e.preventDefault();

    if (
      inputs[0].value == "" ||
      inputs[1].value == "" ||
      inputs[2].value == ""
    ) {
      alert("Please fill the fields first");
    } else {
      updateRecord.style.display = "none";

      var array = {
        sid,
        name: inputs[0].value,
        email: inputs[1].value,
        age: inputs[2].value,
      };

      $.ajax({
        url: "https://rocket-restapi-crud.herokuapp.com/update",
        type: "PUT",
        data: JSON.stringify(array),
        success: function () {
          $("li").each(function (li) {
            if ($(this).data("id") == sid) {
              $(this).children(".name").text(array.name);
              $(this).children(".email").text(array.email);
              $(this).children(".age").text(array.age);

              $("#updated").text("Updated Successfully.");
              $("#updated").show();
              $("#updated").css("background-color", "green");
              setTimeout(function () {
                $("#updated").hide();
              }, 3000);
            }
          });
        },
        error: function (e) {
          $("#updated").text("Updation failed.");
          $("#updated").show();
          $("#updated").css("background-color", "red");
          setTimeout(function () {
            $("#updated").hide();
          }, 3000);
        },
      });
    }
  });

  //---------------------------delete---------------------
  $(".ul").on("click", "strong", function (e) {
    e.stopPropagation();
    var clickedId = $(this).parent().data("id");
    let url = "https://rocket-restapi-crud.herokuapp.com/delete/" + clickedId;

    let removeData = $(this).parent();

    $.ajax({
      method: "DELETE",
      url: url,
    })
      .then(function () {
        removeData.remove();
        $("#deleted").text("Deleted Successfully.");
        $("#deleted").show();
        $("#deleted").css("background-color", "green");
        setTimeout(function () {
          $("#deleted").hide();
        }, 3000);
      })
      .catch(function (e) {
        $("#deleted").text("Error while Deleting.");
        $("#deleted").show();
        $("#deleted").css("background-color", "red");
        setTimeout(function () {
          $("#deleted").hide();
        }, 3000);
      });
  });
});

// ------------------adding data in li--------------------
function addSingleData(d, id) {
  var addingData = $(
    '<li class="task"><span class="name"> ' +
      d.name +
      "</span><span class='email'>" +
      d.email +
      "</span><span class='age'>" +
      d.age +
      "</span><button class='edit'>Edit</button><strong>X</strong></li>"
  );
  addingData.data("id", id);
  $("ul").append(addingData);
}
