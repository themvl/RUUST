// code to draw a graph and distribute it (might change)

const canvas = document.querySelector(".myCanvas");
const width = (canvas.width = window.innerWidth);
const height = (canvas.height = window.innerHeight);
const node_size = 20;
const link_strength = 8;
const delta = 0.1;
const push_force = 500;
center_x = width/2;
center_y = height/2;

const ctx = canvas.getContext("2d");
ctx.fillStyle = "rgb(0 0 0)";
ctx.fillRect(0,0,width,height);

nodes = []
links = []

function add_node(nodes, x, y) {
  nodes.push({x:x,y:y});
  return nodes.length-1
}

function link_nodes(links, link_1, link_2) {
  links.push([link_1, link_2]);
}

function calc_center(nodes) {
  center_x = 0;
  center_y =0;
  nodes.forEach(node => {
    center_x += node.x;
    center_y += node.y;
  })

  center_x/=nodes.length;
  center_y/=nodes.length;
  center_x = (width/2) -center_x;
  center_y = (height/2)-center_y;
}

function redraw(nodes, links) {
  calc_center(nodes);

  ctx.fillStyle = "rgb(0 0 0)";
  ctx.fillRect(0,0,width,height);

  console.log("test");

  nodes.forEach(element => {
    draw_node(element.x, element.y);
  });

  links.forEach(element => {
    draw_path(nodes[element[0]].x, nodes[element[0]].y, nodes[element[1]].x, nodes[element[1]].y);
  })
}

function draw_node(x, y) {
  ctx.fillStyle = "rgb(255 255 255)";
  ctx.fillRect(x-node_size/2+center_x, y-node_size/2+center_y, node_size,node_size);
}

function draw_path(x1, y1, x2, y2) {
  ctx.strokeStyle = "rgb(255 0 0)";
  //draw lines
  ctx.beginPath();
  ctx.moveTo(x1+center_x,y1+center_y);
  ctx.lineTo(x2+center_x,y2+center_y);
  ctx.stroke();
}

function distance(node1, node2) {
  x = node1.x-node2.x;
  y = node1.y-node2.y;
  return Math.sqrt(x**2+y**2);
}

function difference(node1, node2) {
  return {x:node2.x-node1.x, y: node2.y-node1.y};
}

function link_dir_vec(node1, node2) {
  dist = distance(node1,node2);
  diff = difference(node1,node2);
  return {x: diff.x/dist, y: diff.y/dist};
}

function step(nodes,links) {
  forces = [];
  count = 0;
  nodes.forEach(node => {
    forces[count] = {x:0, y:0};
    // pushing away
    for (let index = 0; index < nodes.length; index++) {
      if (index !== count) {
        push = difference(nodes[count], nodes[index]);
        dist = distance(nodes[count], nodes[index]);
        len = Math.sqrt(push.x**2+push.y**2);
        push.x = (push.x/len)*(push_force/dist);
        push.y = (push.y/len)*(push_force/dist);
        console.log("push {}", push)
        forces[count].x -= push.x;
        forces[count].y -= push.y;
        console.log("force: {}", forces[count]);
      }
    }

    console.log("forces: {}", forces);

    //pulling forces
    links.forEach(link => {
      if(link[0] == count) {
        dir = link_dir_vec(nodes[link[0]], nodes[link[1]]);
        console.log("pull: {}", dir);
        forces[count].x += dir.x*link_strength;
        forces[count].y += dir.y*link_strength;
      }
      if(link[1] == count) {
        dir = link_dir_vec(nodes[link[1]], nodes[link[0]]);
        console.log("pull: {}", dir);
        forces[count].x += dir.x*link_strength;
        forces[count].y += dir.y*link_strength;
      }
    })

    count++;
  })

  for (let index = 0; index < nodes.length; index++) {
    node = nodes[index];
    force = forces[index];

    node.x += force.x*delta;
    node.y += force.y*delta;
    
  }
}

node_1 = add_node(nodes, 50, 50);
node_2 = add_node(nodes, 50,100);
node_3 = add_node(nodes, 100 ,100);
node_4 = add_node(nodes, 100,200);
node_5 = add_node(nodes, 0,0);
node_6 = add_node(nodes, 50,70);

link_nodes(links, node_1, node_2);
link_nodes(links, node_2, node_3);
link_nodes(links, node_1, node_3);
link_nodes(links, node_1, node_4);
link_nodes(links, node_5, node_3);
link_nodes(links, node_1, node_6);

  step(nodes,links);
  redraw(nodes,links);

function update() {
  step(nodes,links);
  redraw(nodes,links);
  setTimeout(update, 100);
}


update();