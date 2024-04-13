import { Test, type TestingModule } from '@nestjs/testing'
import { PingController } from './ping.controller'
import { PingService } from './ping.service'

describe('PingController', () => {
  let controller: PingController

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      controllers: [PingController],
      providers: [PingService],
    }).compile()

    controller = module.get<PingController>(PingController)
  })

  it('should be defined', () => {
    expect(controller).toBeDefined()
  })

  it('Should Return None', () => {
    expect(controller.ping().data.length).toBe(0)
  })

  it('Create and do stuff', () => {
    const id = 2
    const created = controller.pongCreate(2).data

    expect(controller.ping().data.length).toBe(1)
    const found = controller.ping().data.at(0)
    expect(found[0]).toBe(id)
    expect(found[1]).toBe(created)

    const gotten = controller.pingId(id).data
    expect(gotten).toBe(created)
  })
})
