import * as pinia from "pinia";
import * as models from "../models";
import * as vue from "vue";

type Platform = "nintendo" | "playstation" | "steam" | "xbox";
type Id = "gui";
type State = models.gui.Gui & { readonly focusedPlatform: Platform | null };
type Getters = Record<string, never>;
type Actions = {
  focusPlatform(platform: Platform | null): void;
  unfocusPlatform(platform: Platform): void;
};

export type Store = pinia.Store<Id, State, Getters, Actions>;
export type StoreDefinition = pinia.StoreDefinition<Id, State, Getters, Actions>;

export const useStore: StoreDefinition = pinia.defineStore("gui", () => {
  const writableState = vue.reactive(models.Gui.make());
  const readOnlyState = vue.reactive({
    focusedPlatform: null as Platform | null,
  });
  const actions: Actions = {
    focusPlatform(this: typeof writableState & typeof readOnlyState, platform) {
      this.focusedPlatform = platform;
    },
    unfocusPlatform(this: typeof writableState & typeof readOnlyState, platform) {
      if (platform !== "nintendo" && this.services.nintendo.enabled) {
        this.focusedPlatform = "nintendo";
        return;
      }
      if (platform !== "playstation" && this.services.playstation.enabled) {
        this.focusedPlatform = "playstation";
        return;
      }
      if (platform !== "steam" && this.services.steam.enabled) {
        this.focusedPlatform = "steam";
        return;
      }
      if (platform !== "xbox" && this.services.xbox.enabled) {
        this.focusedPlatform = "xbox";
        return;
      }
      this.focusedPlatform = null;
    },
  };
  return {
    ...vue.toRefs(writableState),
    ...vue.toRefs(vue.readonly(readOnlyState)),
    ...actions,
  };
});
