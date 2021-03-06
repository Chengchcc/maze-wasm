import ('../pkg/index.js').then(({init_maze, gen_maze, cal_path})=>{
    let col = 60,
        row = 60 
    console.time("generate maze");
    init_maze(BigInt(col), BigInt(row))
    const linkedMap = gen_maze()
    console.timeEnd("generate maze");
    draw(col, row, linkedMap)

    const path = cal_path()
    drawPath(col, row, path)
})

function drawBorder(cols, rows){
    const canvas = document.getElementById('canvas')
    const cellWidth =  canvas.width / cols,
        cellHeight = canvas.height / rows,
        ctx = canvas.getContext('2d')
    ctx.translate(0.5, 0.5)
    ctx.beginPath(  )
    // 上
    ctx.moveTo(0, 0)
    ctx.lineTo(cols * cellWidth >> 0, 0)

    // 下
    ctx.moveTo(0, (rows * cellHeight >>0) - 1 )
    ctx.lineTo((cols * cellWidth >> 0) - 1, (rows * cellHeight >>0) -1 )

    // 左
    ctx.moveTo(0, cellHeight>>0)
    ctx.lineTo(0, (rows * cellHeight >>0) - 1 )

    // 右
    ctx.moveTo((cols * cellWidth >> 0) - 1, 0)
    ctx.lineTo((cols * cellWidth >> 0) - 1, ((rows -1) * cellHeight >>0) -1 )
    ctx.strokeStyle = 'black'
    ctx.stroke()
}


function draw(cols, rows, linkedMap){
    const canvas = document.getElementById('canvas')
    const {height, width} = canvas.getBoundingClientRect()
    canvas.height = height
    canvas.width = width
    const cellWidth =  canvas.width / cols,
        cellHeight = canvas.height / rows,
        cells = cols * rows,
        ctx = canvas.getContext('2d')

    ctx.translate(0.5, 0.5)
    for(var i = 0; i < cells; i++){
        var row = i / cols >> 0,
            column = i % cols;
        //画右边的竖线
        if(column !== cols - 1 && (!linkedMap[i] || linkedMap[i].indexOf(i + 1) < 0)){
            ctx.moveTo((column + 1) * cellWidth >> 0, row * cellHeight >> 0);
            ctx.lineTo((column + 1) * cellWidth >> 0, (row + 1) * cellHeight >> 0);
        }
        //画下面的横线
        if(row !== rows - 1 && (!linkedMap[i] || linkedMap[i].indexOf(i + cols) < 0)){
            ctx.moveTo(column * cellWidth >> 0, (row + 1) * cellHeight >> 0);
            ctx.lineTo((column + 1) * cellWidth >> 0, (row + 1) * cellHeight >> 0);
        }
    }

    ctx.strokeStyle = 'black'

    //最后再一次性stroke，提高性能
    ctx.stroke();


    drawBorder(cols, rows)

    ctx.stroke()

}

function getPoint(cols, rows, cellWidth, cellHeight, cell){
    const row = cell / rows >> 0,
        col = cell % cols
   const p = {
        x:  (col * cellWidth + cellWidth/2) >> 0,
        y:  (row * cellHeight + cellHeight / 2) >> 0
   }
   return Object.values(p)
}

function drawPath(cols, rows, path) {
    const canvas = document.getElementById('canvas')
    const cellWidth =  canvas.width / cols,
        cellHeight = canvas.height / rows,
        ctx = canvas.getContext('2d')

    // ctx.beginPath()
    ctx.moveTo(0, cellHeight/2)
    while (path.length) {
        let p = path.pop()
        ctx.lineTo(...getPoint(cols, rows, cellWidth, cellHeight, p))
    }
    ctx.lineTo(cols * cellWidth - 1 , rows* cellHeight - cellHeight/2)
    ctx.strokeStyle = 'red'
    // ctx.closePath()
    ctx.stroke()
}
